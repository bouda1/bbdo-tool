extern crate quick_protobuf;

mod bbdo;
pub mod event;

pub mod pb_bbdo;

use pb_bbdo::com::centreon::broker::Host;
use pb_bbdo::com::centreon::broker::ServiceStatus;

use quick_protobuf::Reader;

use bbdo::Bbdo;
use clap::App;
use colored::*;
use phf::phf_map;
use serde_json::Value;
use std::process;

static EVENT: phf::Map<&'static str, i32> = phf_map! {
    "NEB::Acknowledgement" => 0x00010001,
    "NEB::Comment" => 0x00010002,
    "NEB::CustomVariable" => 0x00010003,
    "NEB::CustomVariableStatus" => 0x00010004,
    "NEB::Downtime" => 0x00010005,
    "NEB::EventHandler" => 0x00010006,
    "NEB::FlappingStatus" => 0x00010007,
    "NEB::HostCheck" => 0x00010008,
    "NEB::HostDependency" => 0x00010009,
    "NEB::HostGroup" => 0x0001000a,
    "NEB::HostGroupMember" => 0x0001000b,
    "NEB::Host" => 0x0001000c,
    "NEB::HostParent" => 0x0001000d,
    "NEB::HostStatus" => 0x0001000e,
    "NEB::Instance" => 0x0001000f,
    "NEB::InstanceStatus" => 0x00010010,
    "NEB::LogEntry" => 0x00010011,
    "NEB::Module" => 0x00010012,
    "NEB::ServiceCheck" => 0x00010013,
    "NEB::ServiceDependency" => 0x00010014,
    "NEB::ServiceGroup" => 0x00010015,
    "NEB::ServiceGroupMember" => 0x00010016,
    "NEB::Service" => 0x00010017,
    "NEB::ServiceStatus" => 0x00010018,
    "NEB::InstanceConfiguration" => 0x00010019,
    "NEB::ResponsiveInstance" => 0x0001001a,
    "NEB::PbService" => 0x0001001b,
    "NEB::PbAdaptiveService" => 0x0001001c,
    "PbServiceStatus" => 0x0001001d,
    "PbHost" => 0x0001001e,
    "PbAdaptiveHost" => 0x0001001f,
    "PbHostStatus" => 0x00010020,
    "PbSeverity" => 0x00010021,
    "PbTag" => 0x00010022,
    "PbComment" => 0x00010023,
    "PbDowntime" => 0x00010024,
    "PbCustom_variable" => 0x00010025,
    "PbCustom_variable_status" => 0x00010026,
    "PbHost_check" => 0x00010027,
    "PbService_check" => 0x00010028,
    "PbLogEntry" => 0x00010029,
    "PbInstanceStatus" => 0x0001002a,
    "PbModule" => 0x0001002b,
    "PbInstance" => 0x0001002c,
    "PbAcknowledgement" => 0x0001002d,
    "PbResponsive_instance" => 0x0001002e,
    "Storage::Metric" => 0x00030001,
    "Storage::Rebuild" => 0x00030002,
    "Storage::RemoveGraph" => 0x00030003,
    "Storage::Status" => 0x00030004,
    "Storage::IndexMapping" => 0x00030005,
    "Storage::MetricMapping" => 0x00030006,
    "Storage::RebuildMessage" => 0x00030007,
    "Storage::RemoveGraphMessage" => 0x00030008,
    "Storage::PbMetric" => 0x00030009,
    "Storage::PbStatus" => 0x0003000a,
    "Storage::PbIndexMapping" => 0x0003000b,
    "Storage::PbMetricMapping" => 0x0003000c,
    "BBDO::VersionResponse" => 0xffff0001,
    "BBDO::Ack" => 0xffff0002,
    "BBDO::Stop" => 0xffff0003,
    "BBDO::RebuildGraphs" => 0xffff0004,
    "BBDO::RemoveGraphs" => 0xffff0005,
    "BBDO::RemovePoller" => 0xffff0006,
    "BBDO::Welcome" => 0xffff0007,
    "BBDO::PbAck" => 0xffff0008,
    "BBDO::PbStop" => 0xffff0009,
    "BAM::BaStatus" => 0x00060001,
    "BAM::KpiStatus" => 0x00060002,
    "BAM::MetaServiceStatus" => 0x00060003,
    "BAM::BaEvent" => 0x00060004,
    "BAM::KpiEvent" => 0x00060005,
    "BAM::BaDurationEvent" => 0x00060006,
    "BAM::DimensionBaEvent" => 0x00060007,
    "BAM::DimensionKpiEvent" => 0x00060008,
    "BAM::DimensionBaBvRelationEvent" => 0x00060009,
    "BAM::DimensionBvEvent" => 0x0006000a,
    "BAM::DimensionTruncateTableSignal" => 0x0006000b,
    "BAM::Rebuild" => 0x0006000c,
    "BAM::DimensionTimeperiod" => 0x0006000d,
    "BAM::DimensionBaTimeperiodRelation" => 0x0006000e,
    "BAM::DimensionTimeperiodException" => 0x0006000f,
    "BAM::DimensionTimeperiodExclusion" => 0x00060010,
    "BAM::InheritedDowntime" => 0x00060011,
    "BAM::PbInheritedDowntime" => 0x00060012,
    "BAM::PbBaStatus" => 0x00060013,
    "BAM::PbBaEvent" => 0x00060014,
    "BAM::PbKpiEvent" => 0x00060015,
    "BAM::PbDimensionBvEvent" => 0x00060016,
    "BAM::PbDimensionBaBvRelationEvent" => 0x00060017,
    "BAM::PbDimensionTimeperiod" => 0x00060018,
    "BAM::PbDimensionBaEvent" => 0x00060019,
    "BAM::PbDimensionKpiEvent" => 0x0006001a,
    "BAM::PbKpiStatus" => 0x0006001b,
    "BAM::PbBaDurationEvent" => 0x0006001c,
    "BAM::PbDimensionBaTimeperiodRelation" => 0x0006001d,
    "BAM::PbDimensionTruncateTableSignal" => 0x0006001e,
    "unknown" => 0,
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
             -d, --deprecated           'This option is useful to read files generated by centreon-broker with version < 20.10'
             -t, --type                 'Returns only the types names of each event'
             -o --output=[OUTPUT]       'The output of the command generates a new BBDO file.'
             <INPUT>                    'Sets the input file to read'",
        )
        .get_matches();

    let count = matches.occurrences_of("count") == 1;
    let typ = matches.occurrences_of("type") == 1;
    let deprecated = matches.occurrences_of("deprecated") == 1;
    let invert = matches.occurrences_of("invert-match") == 1;

    let bbdo_file = matches.value_of("INPUT").unwrap();

    let filter = matches.value_of("filter").unwrap_or("{}");
    let filter_event = matches.value_of("filter-event").unwrap_or("");
    let output = matches.value_of("output").unwrap_or("");

    if count && output != "" {
        println!("You cannot use the --count and --output options at the same time");
        process::exit(3);
    }

    if typ && output != "" {
        println!("You cannot use the --type and --output options at the same time");
        process::exit(3);
    }

    if filter_event != "" && !EVENT.contains_key(&filter_event) {
        println!(
            "{}: The event filter '{}' does not exist",
            "Error".red().bold(),
            filter_event.green()
        );
        println!("Available events are:");
        let mut events = Vec::new();
        for t in &EVENT {
            events.push(t);
        }
        events.sort();
        for t in &events {
            println!(" * {}", t.0.green());
        }

        process::exit(2);
    }
    let key_event = if filter_event == "" {
        -1
    } else {
        EVENT[&filter_event]
    };
    if invert && filter == "{}" {
        println!(
            "{}: You cannot invert match without filter.",
            "Warning".yellow().bold()
        );
    }

    let filter: Value = match serde_json::from_str(filter) {
        Ok(v) => v,
        Err(_e) => {
            println!("{}: The filter is not a json string", "Error".red().bold());
            process::exit(1);
        }
    };

    let filter = filter.as_object().unwrap();

    let mut b = Bbdo::new(&bbdo_file, &output);

    // retval is only used with the --count option.
    let mut retval = 0;

    let compressed = b.is_compressed();
    while b.offset < b.len() {
        let res = b.deserialize(&compressed, &key_event, &deprecated);

        if res.is_err() {
            continue;
        }
        let res = res.unwrap();
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
                    if typ {
                        println!("{}", res_name);
                    } else {
                        println!("{}", res.to_string());
                    }
                }
            }
        }
    }
    if count {
        println!("{} events", retval);
    }
}
