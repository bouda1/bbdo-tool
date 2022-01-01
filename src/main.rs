mod bbdo;
pub mod event;

use clap::App;
use phf::phf_set;
use serde_json::Value;
use bbdo::Bbdo;
use std::process;
use colored::*;

static EVENT: phf::Set<&'static str> = phf_set! {
    "NEB::Acknowledgement",
    "NEB::Comment",
    "NEB::CustomVariable",
    "NEB::CustomVariableStatus",
    "NEB::Downtime",
    "NEB::EventHandler",
    "NEB::FlappingStatus",
    "NEB::HostCheck",
    "NEB::HostDependency",
    "NEB::HostGroup",
    "NEB::HostGroupMember",
    "NEB::Host",
    "NEB::HostParent",
    "NEB::HostStatus",
    "NEB::Instance",
    "NEB::InstanceStatus",
    "NEB::LogEntry",
    "NEB::Module",
    "NEB::ServiceCheck",
    "NEB::ServiceDependency",
    "NEB::ServiceGroup",
    "NEB::ServiceGroupMember",
    "NEB::Service",
    "NEB::ServiceStatus",
    "NEB::InstanceConfiguration",
    "Storage::Metric",
    "Storage::Rebuild",
    "Storage::RemoveGraph",
    "Storage::Status",
    "Storage::IndexMapping",
    "Storage::MetricMapping",
    "BBDO::VersionResponse",
    "BBDO::Ack",
    "BAM::BaStatus",
    "BAM::KpiStatus",
    "BAM::MetaServiceStatus",
    "BAM::BaEvent",
    "BAM::KpiEvent",
    "BAM::BaDurationEvent",
    "BAM::DimensionBaEvent",
    "BAM::DimensionKpiEvent",
    "BAM::DimensionBaBvRelationEvent",
    "BAM::DimensionBvEvent",
    "BAM::DimensionTruncateTableSignal",
    "BAM::Rebuild",
    "BAM::DimensionTimeperiod",
    "BAM::DimensionBaTimeperiodRelation",
    "BAM::DimensionTimeperiodException",
    "BAM::DimensionTimeperiodExclusion",
    "BAM::InheritedDowntime",
    "unknown",
};

fn main() {
    let matches = App::new("bbdo-tool")
        .version("1.0")
        .author("David Boucher <boudav@gmail.com>")
        .about("display, filter BBDO files content")
        .args_from_usage(
            "-c, --count                'Returns the number of events'
             -v, --invert-match         'Select lines without match'
             -e, --filter-event=[NAME]  'Returns only events of type NAME'
             -f, --filter=[JSON]        'Returns only events with keys matching the ones of JSON'
             <INPUT>                    'Sets the input file to read'",
        )
        .get_matches();

    let count = matches.occurrences_of("count") == 1;
    let invert = matches.occurrences_of("invert-match") == 1;

    let bbdo_file = matches.value_of("INPUT").unwrap();

    let filter = matches.value_of("filter").unwrap_or("{}");
    let filter_event = matches.value_of("filter-event").unwrap_or("");

    if filter_event != "" && !EVENT.contains(&filter_event) {
        println!("{}: The event filter '{}' does not exist", "Error".red().bold(), filter_event.green());
        println!("Available events are:");
        let mut events = Vec::new();
        for t in &EVENT {
            events.push(t);
        }
        events.sort();
        for t in &events {
            println!(" * {}", t.green());
        }

        process::exit(2);
    }
    if invert && filter == "{}" {
        println!("{}: You cannot invert match without filter.", "Warning".yellow().bold());
    }

    let filter : Value = match serde_json::from_str(filter) {
        Ok(v) => v,
        Err(_e) => {
            println!("{}: The filter is not a json string", "Error".red().bold());
            process::exit(1);
        }
    };

    let filter = filter.as_object().unwrap();

    let mut b = Bbdo::new(&bbdo_file);

    // retval is only used with the --count option.
    let mut retval = 0;

    let compressed = b.is_compressed();
    while b.offset < b.len() {
        let res = b.deserialize(&compressed);
        let res_name = res[0].as_str().unwrap();
        let res_obj = res[1].as_object().unwrap();

        let mut dont = false;
        for (key, value) in filter.iter() {
            if !res_obj.contains_key(key) || res[1][&key] != *value {
                dont = true;
                break;
            }
        }

        if (!dont && !invert) || (dont && invert) {
            if filter_event == "" || res_name == filter_event {
                if count {
                    retval += 1;
                } else {
                    println!("{}", res.to_string());
                }
            }
        }
    }
    if count {
        println!("{} events", retval);
    }
}
