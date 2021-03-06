extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate trackable;

use rustracing::tag::Tag;
use rustracing_jaeger::reporter::JaegerCompactReporter;
use rustracing_jaeger::Tracer;
use std::thread;
use std::time::Duration;

fn main() {
    let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
    {
        let span0 = tracer.span("main").start();
        thread::sleep(Duration::from_millis(10));
        {
            let mut span1 = tracer
                .span("sub")
                .child_of(&span0)
                .tag(Tag::new("foo", "bar"))
                .start();
            span1.log(|log| {
                log.error().message("something wrong");
            });
            thread::sleep(Duration::from_millis(10));
        }
    }

    let mut reporter = track_try_unwrap!(JaegerCompactReporter::new("example"));
    reporter.add_service_tag(Tag::new("hello", "world"));
    track_try_unwrap!(reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
}
