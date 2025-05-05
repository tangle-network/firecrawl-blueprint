use blueprint_sdk::Job;
use blueprint_sdk::Router;
use blueprint_sdk::contexts::tangle::TangleClientContext;
use blueprint_sdk::crypto::sp_core::SpSr25519;
use blueprint_sdk::crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment as SdkBlueprintEnvironment; // Alias SDK env
use blueprint_sdk::runner::tangle::config::TangleConfig;
use blueprint_sdk::tangle::consumer::TangleConsumer;
use blueprint_sdk::tangle::filters::MatchesServiceId;
use blueprint_sdk::tangle::layers::TangleLayer;
use blueprint_sdk::tangle::producer::TangleProducer;
// Import new context, env, handlers, and IDs from the library
use firecrawl_blueprint_blueprint_lib::{
    JOB_BATCH_SCRAPE_ID,
    JOB_CRAWL_ID,
    JOB_MAP_ID,
    JOB_SCRAPE_ID,
    context::{BlueprintEnvironment, Context}, // Use library's Context and Env
    handle_batch_scrape,
    handle_crawl,
    handle_map,
    handle_scrape,
};
use tower::filter::FilterLayer;
use tracing::error;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    let sdk_env = SdkBlueprintEnvironment::load()?; // Load SDK env first
    let firecrawl_env = BlueprintEnvironment::from_env(); // Load library env
    let sr25519_signer = sdk_env.keystore().first_local::<SpSr25519>()?;
    let sr25519_pair = sdk_env
        .keystore()
        .get_secret::<SpSr25519>(&sr25519_signer)?;
    let st25519_signer = TanglePairSigner::new(sr25519_pair.0);

    let tangle_client = sdk_env.tangle_client().await?;
    let tangle_producer =
        TangleProducer::finalized_blocks(tangle_client.rpc_client.clone()).await?;
    let tangle_consumer = TangleConsumer::new(tangle_client.rpc_client.clone(), st25519_signer);

    let tangle_config = TangleConfig::default();

    let service_id = sdk_env.protocol_settings.tangle()?.service_id.unwrap();
    let result = BlueprintRunner::builder(tangle_config, sdk_env) // Use SDK env for runner builder
        .router(
            // A router
            //
            // Each "route" is a job ID and the job function. We can also support arbitrary `Service`s from `tower`,
            // which may make it easier for people to port over existing services to a blueprint.
            Router::new()
                // Add routes for the new Firecrawl jobs with TangleLayer
                .route(JOB_SCRAPE_ID, handle_scrape.layer(TangleLayer))
                .route(JOB_CRAWL_ID, handle_crawl.layer(TangleLayer))
                .route(JOB_BATCH_SCRAPE_ID, handle_batch_scrape.layer(TangleLayer))
                .route(JOB_MAP_ID, handle_map.layer(TangleLayer))
                // Add the global FilterLayer for service ID matching
                .layer(FilterLayer::new(MatchesServiceId(service_id)))
                // Add the library's context, initialized with the library's environment
                .with_context(Context::new(firecrawl_env)),
        )
        // Add potentially many producers
        //
        // A producer is simply a `Stream` that outputs `JobCall`s, which are passed down to the intended
        // job functions.
        .producer(tangle_producer)
        // Add potentially many consumers
        //
        // A consumer is simply a `Sink` that consumes `JobResult`s, which are the output of the job functions.
        // Every result will be passed to every consumer. It is the responsibility of the consumer
        // to determine whether or not to process a result.
        .consumer(tangle_consumer)
        // Custom shutdown handlers
        //
        // Now users can specify what to do when an error occurs and the runner is shutting down.
        // That can be cleanup logic, finalizing database transactions, etc.
        .with_shutdown_handler(async { println!("Shutting down!") })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed! {e:?}");
    }

    Ok(())
}

pub fn setup_log() {
    use tracing_subscriber::util::SubscriberInitExt;

    let _ = tracing_subscriber::fmt::SubscriberBuilder::default()
        .without_time()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE)
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish()
        .try_init();
}
