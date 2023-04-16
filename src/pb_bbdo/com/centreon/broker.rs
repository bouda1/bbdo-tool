// Automatically generated rust module for 'neb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TagType {
    SERVICEGROUP = 0,
    HOSTGROUP = 1,
    SERVICECATEGORY = 2,
    HOSTCATEGORY = 3,
}

impl Default for TagType {
    fn default() -> Self {
        TagType::SERVICEGROUP
    }
}

impl From<i32> for TagType {
    fn from(i: i32) -> Self {
        match i {
            0 => TagType::SERVICEGROUP,
            1 => TagType::HOSTGROUP,
            2 => TagType::SERVICECATEGORY,
            3 => TagType::HOSTCATEGORY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TagType {
    fn from(s: &'a str) -> Self {
        match s {
            "SERVICEGROUP" => TagType::SERVICEGROUP,
            "HOSTGROUP" => TagType::HOSTGROUP,
            "SERVICECATEGORY" => TagType::SERVICECATEGORY,
            "HOSTCATEGORY" => TagType::HOSTCATEGORY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceType {
    SERVICE = 0,
    METASERVICE = 2,
    BA = 3,
    ANOMALY_DETECTION = 4,
}

impl Default for ServiceType {
    fn default() -> Self {
        ServiceType::SERVICE
    }
}

impl From<i32> for ServiceType {
    fn from(i: i32) -> Self {
        match i {
            0 => ServiceType::SERVICE,
            2 => ServiceType::METASERVICE,
            3 => ServiceType::BA,
            4 => ServiceType::ANOMALY_DETECTION,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ServiceType {
    fn from(s: &'a str) -> Self {
        match s {
            "SERVICE" => ServiceType::SERVICE,
            "METASERVICE" => ServiceType::METASERVICE,
            "BA" => ServiceType::BA,
            "ANOMALY_DETECTION" => ServiceType::ANOMALY_DETECTION,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AckType {
    NONE = 0,
    NORMAL = 1,
    STICKY = 2,
}

impl Default for AckType {
    fn default() -> Self {
        AckType::NONE
    }
}

impl From<i32> for AckType {
    fn from(i: i32) -> Self {
        match i {
            0 => AckType::NONE,
            1 => AckType::NORMAL,
            2 => AckType::STICKY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for AckType {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => AckType::NONE,
            "NORMAL" => AckType::NORMAL,
            "STICKY" => AckType::STICKY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CheckType {
    CheckActive = 0,
    CheckPassive = 1,
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::CheckActive
    }
}

impl From<i32> for CheckType {
    fn from(i: i32) -> Self {
        match i {
            0 => CheckType::CheckActive,
            1 => CheckType::CheckPassive,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CheckType {
    fn from(s: &'a str) -> Self {
        match s {
            "CheckActive" => CheckType::CheckActive,
            "CheckPassive" => CheckType::CheckPassive,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tag<'a> {
    pub id: u64,
    pub action: com::centreon::broker::mod_Tag::Action,
    pub type_pb: com::centreon::broker::TagType,
    pub name: Cow<'a, str>,
    pub poller_id: i64,
}

impl<'a> MessageRead<'a> for Tag<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(16) => msg.action = r.read_enum(bytes)?,
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(34) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.poller_id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Tag<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.action == com::centreon::broker::mod_Tag::Action::ADD { 0 } else { 1 + sizeof_varint(*(&self.action) as u64) }
        + if self.type_pb == com::centreon::broker::TagType::SERVICEGROUP { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.poller_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.poller_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if self.action != com::centreon::broker::mod_Tag::Action::ADD { w.write_with_tag(16, |w| w.write_enum(*&self.action as i32))?; }
        if self.type_pb != com::centreon::broker::TagType::SERVICEGROUP { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.name != "" { w.write_with_tag(34, |w| w.write_string(&**&self.name))?; }
        if self.poller_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.poller_id))?; }
        Ok(())
    }
}

pub mod mod_Tag {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    ADD = 0,
    DELETE = 1,
    MODIFY = 2,
}

impl Default for Action {
    fn default() -> Self {
        Action::ADD
    }
}

impl From<i32> for Action {
    fn from(i: i32) -> Self {
        match i {
            0 => Action::ADD,
            1 => Action::DELETE,
            2 => Action::MODIFY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Action {
    fn from(s: &'a str) -> Self {
        match s {
            "ADD" => Action::ADD,
            "DELETE" => Action::DELETE,
            "MODIFY" => Action::MODIFY,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TagInfo {
    pub id: u64,
    pub type_pb: com::centreon::broker::TagType,
}

impl<'a> MessageRead<'a> for TagInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TagInfo {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.type_pb == com::centreon::broker::TagType::SERVICEGROUP { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if self.type_pb != com::centreon::broker::TagType::SERVICEGROUP { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BBDOHeader {
    pub conf_version: u32,
}

impl<'a> MessageRead<'a> for BBDOHeader {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.conf_version = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BBDOHeader {
    fn get_size(&self) -> usize {
        0
        + if self.conf_version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.conf_version) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.conf_version != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.conf_version))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Service<'a> {
    pub host_id: u64,
    pub service_id: u64,
    pub acknowledged: bool,
    pub acknowledgement_type: com::centreon::broker::AckType,
    pub active_checks: bool,
    pub enabled: bool,
    pub scheduled_downtime_depth: i32,
    pub check_command: Cow<'a, str>,
    pub check_interval: u32,
    pub check_period: Cow<'a, str>,
    pub check_type: com::centreon::broker::mod_Service::CheckType,
    pub check_attempt: i32,
    pub state: com::centreon::broker::mod_Service::State,
    pub event_handler_enabled: bool,
    pub event_handler: Cow<'a, str>,
    pub execution_time: f64,
    pub flap_detection: bool,
    pub checked: bool,
    pub flapping: bool,
    pub last_check: i64,
    pub last_hard_state: com::centreon::broker::mod_Service::State,
    pub last_hard_state_change: i64,
    pub last_notification: i64,
    pub notification_number: i32,
    pub last_state_change: i64,
    pub last_time_ok: i64,
    pub last_time_warning: i64,
    pub last_time_critical: i64,
    pub last_time_unknown: i64,
    pub last_update: i64,
    pub latency: f64,
    pub max_check_attempts: u32,
    pub next_check: i64,
    pub next_notification: i64,
    pub no_more_notifications: bool,
    pub notify: bool,
    pub output: Cow<'a, str>,
    pub long_output: Cow<'a, str>,
    pub passive_checks: bool,
    pub percent_state_change: f64,
    pub perfdata: Cow<'a, str>,
    pub retry_interval: f64,
    pub host_name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub should_be_scheduled: bool,
    pub obsess_over_service: bool,
    pub state_type: com::centreon::broker::mod_Service::StateType,
    pub action_url: Cow<'a, str>,
    pub check_freshness: bool,
    pub default_active_checks: bool,
    pub default_event_handler_enabled: bool,
    pub default_flap_detection: bool,
    pub default_notify: bool,
    pub default_passive_checks: bool,
    pub display_name: Cow<'a, str>,
    pub first_notification_delay: f64,
    pub flap_detection_on_critical: bool,
    pub flap_detection_on_ok: bool,
    pub flap_detection_on_unknown: bool,
    pub flap_detection_on_warning: bool,
    pub freshness_threshold: f64,
    pub high_flap_threshold: f64,
    pub icon_image: Cow<'a, str>,
    pub icon_image_alt: Cow<'a, str>,
    pub is_volatile: bool,
    pub low_flap_threshold: f64,
    pub notes: Cow<'a, str>,
    pub notes_url: Cow<'a, str>,
    pub notification_interval: f64,
    pub notification_period: Cow<'a, str>,
    pub notify_on_critical: bool,
    pub notify_on_downtime: bool,
    pub notify_on_flapping: bool,
    pub notify_on_recovery: bool,
    pub notify_on_unknown: bool,
    pub notify_on_warning: bool,
    pub stalk_on_critical: bool,
    pub stalk_on_ok: bool,
    pub stalk_on_unknown: bool,
    pub stalk_on_warning: bool,
    pub retain_nonstatus_information: bool,
    pub retain_status_information: bool,
    pub severity_id: u64,
    pub tags: Vec<com::centreon::broker::TagInfo>,
    pub type_pb: com::centreon::broker::ServiceType,
    pub internal_id: u64,
    pub icon_id: u64,
}

impl<'a> MessageRead<'a> for Service<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.service_id = r.read_uint64(bytes)?,
                Ok(24) => msg.acknowledged = r.read_bool(bytes)?,
                Ok(32) => msg.acknowledgement_type = r.read_enum(bytes)?,
                Ok(40) => msg.active_checks = r.read_bool(bytes)?,
                Ok(48) => msg.enabled = r.read_bool(bytes)?,
                Ok(56) => msg.scheduled_downtime_depth = r.read_int32(bytes)?,
                Ok(66) => msg.check_command = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.check_interval = r.read_uint32(bytes)?,
                Ok(82) => msg.check_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.check_type = r.read_enum(bytes)?,
                Ok(96) => msg.check_attempt = r.read_int32(bytes)?,
                Ok(104) => msg.state = r.read_enum(bytes)?,
                Ok(112) => msg.event_handler_enabled = r.read_bool(bytes)?,
                Ok(122) => msg.event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(129) => msg.execution_time = r.read_double(bytes)?,
                Ok(136) => msg.flap_detection = r.read_bool(bytes)?,
                Ok(144) => msg.checked = r.read_bool(bytes)?,
                Ok(152) => msg.flapping = r.read_bool(bytes)?,
                Ok(160) => msg.last_check = r.read_int64(bytes)?,
                Ok(168) => msg.last_hard_state = r.read_enum(bytes)?,
                Ok(176) => msg.last_hard_state_change = r.read_int64(bytes)?,
                Ok(184) => msg.last_notification = r.read_int64(bytes)?,
                Ok(192) => msg.notification_number = r.read_int32(bytes)?,
                Ok(200) => msg.last_state_change = r.read_int64(bytes)?,
                Ok(208) => msg.last_time_ok = r.read_int64(bytes)?,
                Ok(216) => msg.last_time_warning = r.read_int64(bytes)?,
                Ok(224) => msg.last_time_critical = r.read_int64(bytes)?,
                Ok(232) => msg.last_time_unknown = r.read_int64(bytes)?,
                Ok(240) => msg.last_update = r.read_int64(bytes)?,
                Ok(249) => msg.latency = r.read_double(bytes)?,
                Ok(256) => msg.max_check_attempts = r.read_uint32(bytes)?,
                Ok(264) => msg.next_check = r.read_int64(bytes)?,
                Ok(272) => msg.next_notification = r.read_int64(bytes)?,
                Ok(280) => msg.no_more_notifications = r.read_bool(bytes)?,
                Ok(288) => msg.notify = r.read_bool(bytes)?,
                Ok(298) => msg.output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(306) => msg.long_output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(312) => msg.passive_checks = r.read_bool(bytes)?,
                Ok(321) => msg.percent_state_change = r.read_double(bytes)?,
                Ok(330) => msg.perfdata = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(337) => msg.retry_interval = r.read_double(bytes)?,
                Ok(346) => msg.host_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(354) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(360) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(368) => msg.obsess_over_service = r.read_bool(bytes)?,
                Ok(376) => msg.state_type = r.read_enum(bytes)?,
                Ok(386) => msg.action_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(392) => msg.check_freshness = r.read_bool(bytes)?,
                Ok(400) => msg.default_active_checks = r.read_bool(bytes)?,
                Ok(408) => msg.default_event_handler_enabled = r.read_bool(bytes)?,
                Ok(416) => msg.default_flap_detection = r.read_bool(bytes)?,
                Ok(424) => msg.default_notify = r.read_bool(bytes)?,
                Ok(432) => msg.default_passive_checks = r.read_bool(bytes)?,
                Ok(442) => msg.display_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(449) => msg.first_notification_delay = r.read_double(bytes)?,
                Ok(456) => msg.flap_detection_on_critical = r.read_bool(bytes)?,
                Ok(464) => msg.flap_detection_on_ok = r.read_bool(bytes)?,
                Ok(472) => msg.flap_detection_on_unknown = r.read_bool(bytes)?,
                Ok(480) => msg.flap_detection_on_warning = r.read_bool(bytes)?,
                Ok(489) => msg.freshness_threshold = r.read_double(bytes)?,
                Ok(497) => msg.high_flap_threshold = r.read_double(bytes)?,
                Ok(506) => msg.icon_image = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(514) => msg.icon_image_alt = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(520) => msg.is_volatile = r.read_bool(bytes)?,
                Ok(529) => msg.low_flap_threshold = r.read_double(bytes)?,
                Ok(538) => msg.notes = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(546) => msg.notes_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(553) => msg.notification_interval = r.read_double(bytes)?,
                Ok(562) => msg.notification_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(568) => msg.notify_on_critical = r.read_bool(bytes)?,
                Ok(576) => msg.notify_on_downtime = r.read_bool(bytes)?,
                Ok(584) => msg.notify_on_flapping = r.read_bool(bytes)?,
                Ok(592) => msg.notify_on_recovery = r.read_bool(bytes)?,
                Ok(600) => msg.notify_on_unknown = r.read_bool(bytes)?,
                Ok(608) => msg.notify_on_warning = r.read_bool(bytes)?,
                Ok(616) => msg.stalk_on_critical = r.read_bool(bytes)?,
                Ok(624) => msg.stalk_on_ok = r.read_bool(bytes)?,
                Ok(632) => msg.stalk_on_unknown = r.read_bool(bytes)?,
                Ok(640) => msg.stalk_on_warning = r.read_bool(bytes)?,
                Ok(648) => msg.retain_nonstatus_information = r.read_bool(bytes)?,
                Ok(656) => msg.retain_status_information = r.read_bool(bytes)?,
                Ok(664) => msg.severity_id = r.read_uint64(bytes)?,
                Ok(674) => msg.tags.push(r.read_message::<com::centreon::broker::TagInfo>(bytes)?),
                Ok(680) => msg.type_pb = r.read_enum(bytes)?,
                Ok(688) => msg.internal_id = r.read_uint64(bytes)?,
                Ok(696) => msg.icon_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Service<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.acknowledged == false { 0 } else { 1 + sizeof_varint(*(&self.acknowledged) as u64) }
        + if self.acknowledgement_type == com::centreon::broker::AckType::NONE { 0 } else { 1 + sizeof_varint(*(&self.acknowledgement_type) as u64) }
        + if self.active_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_checks) as u64) }
        + if self.enabled == false { 0 } else { 1 + sizeof_varint(*(&self.enabled) as u64) }
        + if self.scheduled_downtime_depth == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.scheduled_downtime_depth) as u64) }
        + if self.check_command == "" { 0 } else { 1 + sizeof_len((&self.check_command).len()) }
        + if self.check_interval == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.check_interval) as u64) }
        + if self.check_period == "" { 0 } else { 1 + sizeof_len((&self.check_period).len()) }
        + if self.check_type == com::centreon::broker::mod_Service::CheckType::ACTIVE { 0 } else { 1 + sizeof_varint(*(&self.check_type) as u64) }
        + if self.check_attempt == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.check_attempt) as u64) }
        + if self.state == com::centreon::broker::mod_Service::State::OK { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + if self.event_handler_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.event_handler_enabled) as u64) }
        + if self.event_handler == "" { 0 } else { 1 + sizeof_len((&self.event_handler).len()) }
        + if self.execution_time == 0f64 { 0 } else { 2 + 8 }
        + if self.flap_detection == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection) as u64) }
        + if self.checked == false { 0 } else { 2 + sizeof_varint(*(&self.checked) as u64) }
        + if self.flapping == false { 0 } else { 2 + sizeof_varint(*(&self.flapping) as u64) }
        + if self.last_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_check) as u64) }
        + if self.last_hard_state == com::centreon::broker::mod_Service::State::OK { 0 } else { 2 + sizeof_varint(*(&self.last_hard_state) as u64) }
        + if self.last_hard_state_change == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_hard_state_change) as u64) }
        + if self.last_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_notification) as u64) }
        + if self.notification_number == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.notification_number) as u64) }
        + if self.last_state_change == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_state_change) as u64) }
        + if self.last_time_ok == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_ok) as u64) }
        + if self.last_time_warning == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_warning) as u64) }
        + if self.last_time_critical == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_critical) as u64) }
        + if self.last_time_unknown == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_unknown) as u64) }
        + if self.last_update == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_update) as u64) }
        + if self.latency == 0f64 { 0 } else { 2 + 8 }
        + if self.max_check_attempts == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.max_check_attempts) as u64) }
        + if self.next_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_check) as u64) }
        + if self.next_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_notification) as u64) }
        + if self.no_more_notifications == false { 0 } else { 2 + sizeof_varint(*(&self.no_more_notifications) as u64) }
        + if self.notify == false { 0 } else { 2 + sizeof_varint(*(&self.notify) as u64) }
        + if self.output == "" { 0 } else { 2 + sizeof_len((&self.output).len()) }
        + if self.long_output == "" { 0 } else { 2 + sizeof_len((&self.long_output).len()) }
        + if self.passive_checks == false { 0 } else { 2 + sizeof_varint(*(&self.passive_checks) as u64) }
        + if self.percent_state_change == 0f64 { 0 } else { 2 + 8 }
        + if self.perfdata == "" { 0 } else { 2 + sizeof_len((&self.perfdata).len()) }
        + if self.retry_interval == 0f64 { 0 } else { 2 + 8 }
        + if self.host_name == "" { 0 } else { 2 + sizeof_len((&self.host_name).len()) }
        + if self.description == "" { 0 } else { 2 + sizeof_len((&self.description).len()) }
        + if self.should_be_scheduled == false { 0 } else { 2 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.obsess_over_service == false { 0 } else { 2 + sizeof_varint(*(&self.obsess_over_service) as u64) }
        + if self.state_type == com::centreon::broker::mod_Service::StateType::SOFT { 0 } else { 2 + sizeof_varint(*(&self.state_type) as u64) }
        + if self.action_url == "" { 0 } else { 2 + sizeof_len((&self.action_url).len()) }
        + if self.check_freshness == false { 0 } else { 2 + sizeof_varint(*(&self.check_freshness) as u64) }
        + if self.default_active_checks == false { 0 } else { 2 + sizeof_varint(*(&self.default_active_checks) as u64) }
        + if self.default_event_handler_enabled == false { 0 } else { 2 + sizeof_varint(*(&self.default_event_handler_enabled) as u64) }
        + if self.default_flap_detection == false { 0 } else { 2 + sizeof_varint(*(&self.default_flap_detection) as u64) }
        + if self.default_notify == false { 0 } else { 2 + sizeof_varint(*(&self.default_notify) as u64) }
        + if self.default_passive_checks == false { 0 } else { 2 + sizeof_varint(*(&self.default_passive_checks) as u64) }
        + if self.display_name == "" { 0 } else { 2 + sizeof_len((&self.display_name).len()) }
        + if self.first_notification_delay == 0f64 { 0 } else { 2 + 8 }
        + if self.flap_detection_on_critical == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_critical) as u64) }
        + if self.flap_detection_on_ok == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_ok) as u64) }
        + if self.flap_detection_on_unknown == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_unknown) as u64) }
        + if self.flap_detection_on_warning == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_warning) as u64) }
        + if self.freshness_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.high_flap_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.icon_image == "" { 0 } else { 2 + sizeof_len((&self.icon_image).len()) }
        + if self.icon_image_alt == "" { 0 } else { 2 + sizeof_len((&self.icon_image_alt).len()) }
        + if self.is_volatile == false { 0 } else { 2 + sizeof_varint(*(&self.is_volatile) as u64) }
        + if self.low_flap_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.notes == "" { 0 } else { 2 + sizeof_len((&self.notes).len()) }
        + if self.notes_url == "" { 0 } else { 2 + sizeof_len((&self.notes_url).len()) }
        + if self.notification_interval == 0f64 { 0 } else { 2 + 8 }
        + if self.notification_period == "" { 0 } else { 2 + sizeof_len((&self.notification_period).len()) }
        + if self.notify_on_critical == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_critical) as u64) }
        + if self.notify_on_downtime == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_downtime) as u64) }
        + if self.notify_on_flapping == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_flapping) as u64) }
        + if self.notify_on_recovery == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_recovery) as u64) }
        + if self.notify_on_unknown == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_unknown) as u64) }
        + if self.notify_on_warning == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_warning) as u64) }
        + if self.stalk_on_critical == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_critical) as u64) }
        + if self.stalk_on_ok == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_ok) as u64) }
        + if self.stalk_on_unknown == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_unknown) as u64) }
        + if self.stalk_on_warning == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_warning) as u64) }
        + if self.retain_nonstatus_information == false { 0 } else { 2 + sizeof_varint(*(&self.retain_nonstatus_information) as u64) }
        + if self.retain_status_information == false { 0 } else { 2 + sizeof_varint(*(&self.retain_status_information) as u64) }
        + if self.severity_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.severity_id) as u64) }
        + self.tags.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.type_pb == com::centreon::broker::ServiceType::SERVICE { 0 } else { 2 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.internal_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.internal_id) as u64) }
        + if self.icon_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.icon_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.service_id))?; }
        if self.acknowledged != false { w.write_with_tag(24, |w| w.write_bool(*&self.acknowledged))?; }
        if self.acknowledgement_type != com::centreon::broker::AckType::NONE { w.write_with_tag(32, |w| w.write_enum(*&self.acknowledgement_type as i32))?; }
        if self.active_checks != false { w.write_with_tag(40, |w| w.write_bool(*&self.active_checks))?; }
        if self.enabled != false { w.write_with_tag(48, |w| w.write_bool(*&self.enabled))?; }
        if self.scheduled_downtime_depth != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.scheduled_downtime_depth))?; }
        if self.check_command != "" { w.write_with_tag(66, |w| w.write_string(&**&self.check_command))?; }
        if self.check_interval != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.check_interval))?; }
        if self.check_period != "" { w.write_with_tag(82, |w| w.write_string(&**&self.check_period))?; }
        if self.check_type != com::centreon::broker::mod_Service::CheckType::ACTIVE { w.write_with_tag(88, |w| w.write_enum(*&self.check_type as i32))?; }
        if self.check_attempt != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.check_attempt))?; }
        if self.state != com::centreon::broker::mod_Service::State::OK { w.write_with_tag(104, |w| w.write_enum(*&self.state as i32))?; }
        if self.event_handler_enabled != false { w.write_with_tag(112, |w| w.write_bool(*&self.event_handler_enabled))?; }
        if self.event_handler != "" { w.write_with_tag(122, |w| w.write_string(&**&self.event_handler))?; }
        if self.execution_time != 0f64 { w.write_with_tag(129, |w| w.write_double(*&self.execution_time))?; }
        if self.flap_detection != false { w.write_with_tag(136, |w| w.write_bool(*&self.flap_detection))?; }
        if self.checked != false { w.write_with_tag(144, |w| w.write_bool(*&self.checked))?; }
        if self.flapping != false { w.write_with_tag(152, |w| w.write_bool(*&self.flapping))?; }
        if self.last_check != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.last_check))?; }
        if self.last_hard_state != com::centreon::broker::mod_Service::State::OK { w.write_with_tag(168, |w| w.write_enum(*&self.last_hard_state as i32))?; }
        if self.last_hard_state_change != 0i64 { w.write_with_tag(176, |w| w.write_int64(*&self.last_hard_state_change))?; }
        if self.last_notification != 0i64 { w.write_with_tag(184, |w| w.write_int64(*&self.last_notification))?; }
        if self.notification_number != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.notification_number))?; }
        if self.last_state_change != 0i64 { w.write_with_tag(200, |w| w.write_int64(*&self.last_state_change))?; }
        if self.last_time_ok != 0i64 { w.write_with_tag(208, |w| w.write_int64(*&self.last_time_ok))?; }
        if self.last_time_warning != 0i64 { w.write_with_tag(216, |w| w.write_int64(*&self.last_time_warning))?; }
        if self.last_time_critical != 0i64 { w.write_with_tag(224, |w| w.write_int64(*&self.last_time_critical))?; }
        if self.last_time_unknown != 0i64 { w.write_with_tag(232, |w| w.write_int64(*&self.last_time_unknown))?; }
        if self.last_update != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.last_update))?; }
        if self.latency != 0f64 { w.write_with_tag(249, |w| w.write_double(*&self.latency))?; }
        if self.max_check_attempts != 0u32 { w.write_with_tag(256, |w| w.write_uint32(*&self.max_check_attempts))?; }
        if self.next_check != 0i64 { w.write_with_tag(264, |w| w.write_int64(*&self.next_check))?; }
        if self.next_notification != 0i64 { w.write_with_tag(272, |w| w.write_int64(*&self.next_notification))?; }
        if self.no_more_notifications != false { w.write_with_tag(280, |w| w.write_bool(*&self.no_more_notifications))?; }
        if self.notify != false { w.write_with_tag(288, |w| w.write_bool(*&self.notify))?; }
        if self.output != "" { w.write_with_tag(298, |w| w.write_string(&**&self.output))?; }
        if self.long_output != "" { w.write_with_tag(306, |w| w.write_string(&**&self.long_output))?; }
        if self.passive_checks != false { w.write_with_tag(312, |w| w.write_bool(*&self.passive_checks))?; }
        if self.percent_state_change != 0f64 { w.write_with_tag(321, |w| w.write_double(*&self.percent_state_change))?; }
        if self.perfdata != "" { w.write_with_tag(330, |w| w.write_string(&**&self.perfdata))?; }
        if self.retry_interval != 0f64 { w.write_with_tag(337, |w| w.write_double(*&self.retry_interval))?; }
        if self.host_name != "" { w.write_with_tag(346, |w| w.write_string(&**&self.host_name))?; }
        if self.description != "" { w.write_with_tag(354, |w| w.write_string(&**&self.description))?; }
        if self.should_be_scheduled != false { w.write_with_tag(360, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.obsess_over_service != false { w.write_with_tag(368, |w| w.write_bool(*&self.obsess_over_service))?; }
        if self.state_type != com::centreon::broker::mod_Service::StateType::SOFT { w.write_with_tag(376, |w| w.write_enum(*&self.state_type as i32))?; }
        if self.action_url != "" { w.write_with_tag(386, |w| w.write_string(&**&self.action_url))?; }
        if self.check_freshness != false { w.write_with_tag(392, |w| w.write_bool(*&self.check_freshness))?; }
        if self.default_active_checks != false { w.write_with_tag(400, |w| w.write_bool(*&self.default_active_checks))?; }
        if self.default_event_handler_enabled != false { w.write_with_tag(408, |w| w.write_bool(*&self.default_event_handler_enabled))?; }
        if self.default_flap_detection != false { w.write_with_tag(416, |w| w.write_bool(*&self.default_flap_detection))?; }
        if self.default_notify != false { w.write_with_tag(424, |w| w.write_bool(*&self.default_notify))?; }
        if self.default_passive_checks != false { w.write_with_tag(432, |w| w.write_bool(*&self.default_passive_checks))?; }
        if self.display_name != "" { w.write_with_tag(442, |w| w.write_string(&**&self.display_name))?; }
        if self.first_notification_delay != 0f64 { w.write_with_tag(449, |w| w.write_double(*&self.first_notification_delay))?; }
        if self.flap_detection_on_critical != false { w.write_with_tag(456, |w| w.write_bool(*&self.flap_detection_on_critical))?; }
        if self.flap_detection_on_ok != false { w.write_with_tag(464, |w| w.write_bool(*&self.flap_detection_on_ok))?; }
        if self.flap_detection_on_unknown != false { w.write_with_tag(472, |w| w.write_bool(*&self.flap_detection_on_unknown))?; }
        if self.flap_detection_on_warning != false { w.write_with_tag(480, |w| w.write_bool(*&self.flap_detection_on_warning))?; }
        if self.freshness_threshold != 0f64 { w.write_with_tag(489, |w| w.write_double(*&self.freshness_threshold))?; }
        if self.high_flap_threshold != 0f64 { w.write_with_tag(497, |w| w.write_double(*&self.high_flap_threshold))?; }
        if self.icon_image != "" { w.write_with_tag(506, |w| w.write_string(&**&self.icon_image))?; }
        if self.icon_image_alt != "" { w.write_with_tag(514, |w| w.write_string(&**&self.icon_image_alt))?; }
        if self.is_volatile != false { w.write_with_tag(520, |w| w.write_bool(*&self.is_volatile))?; }
        if self.low_flap_threshold != 0f64 { w.write_with_tag(529, |w| w.write_double(*&self.low_flap_threshold))?; }
        if self.notes != "" { w.write_with_tag(538, |w| w.write_string(&**&self.notes))?; }
        if self.notes_url != "" { w.write_with_tag(546, |w| w.write_string(&**&self.notes_url))?; }
        if self.notification_interval != 0f64 { w.write_with_tag(553, |w| w.write_double(*&self.notification_interval))?; }
        if self.notification_period != "" { w.write_with_tag(562, |w| w.write_string(&**&self.notification_period))?; }
        if self.notify_on_critical != false { w.write_with_tag(568, |w| w.write_bool(*&self.notify_on_critical))?; }
        if self.notify_on_downtime != false { w.write_with_tag(576, |w| w.write_bool(*&self.notify_on_downtime))?; }
        if self.notify_on_flapping != false { w.write_with_tag(584, |w| w.write_bool(*&self.notify_on_flapping))?; }
        if self.notify_on_recovery != false { w.write_with_tag(592, |w| w.write_bool(*&self.notify_on_recovery))?; }
        if self.notify_on_unknown != false { w.write_with_tag(600, |w| w.write_bool(*&self.notify_on_unknown))?; }
        if self.notify_on_warning != false { w.write_with_tag(608, |w| w.write_bool(*&self.notify_on_warning))?; }
        if self.stalk_on_critical != false { w.write_with_tag(616, |w| w.write_bool(*&self.stalk_on_critical))?; }
        if self.stalk_on_ok != false { w.write_with_tag(624, |w| w.write_bool(*&self.stalk_on_ok))?; }
        if self.stalk_on_unknown != false { w.write_with_tag(632, |w| w.write_bool(*&self.stalk_on_unknown))?; }
        if self.stalk_on_warning != false { w.write_with_tag(640, |w| w.write_bool(*&self.stalk_on_warning))?; }
        if self.retain_nonstatus_information != false { w.write_with_tag(648, |w| w.write_bool(*&self.retain_nonstatus_information))?; }
        if self.retain_status_information != false { w.write_with_tag(656, |w| w.write_bool(*&self.retain_status_information))?; }
        if self.severity_id != 0u64 { w.write_with_tag(664, |w| w.write_uint64(*&self.severity_id))?; }
        for s in &self.tags { w.write_with_tag(674, |w| w.write_message(s))?; }
        if self.type_pb != com::centreon::broker::ServiceType::SERVICE { w.write_with_tag(680, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.internal_id != 0u64 { w.write_with_tag(688, |w| w.write_uint64(*&self.internal_id))?; }
        if self.icon_id != 0u64 { w.write_with_tag(696, |w| w.write_uint64(*&self.icon_id))?; }
        Ok(())
    }
}

pub mod mod_Service {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CheckType {
    ACTIVE = 0,
    PASSIVE = 1,
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::ACTIVE
    }
}

impl From<i32> for CheckType {
    fn from(i: i32) -> Self {
        match i {
            0 => CheckType::ACTIVE,
            1 => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CheckType {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTIVE" => CheckType::ACTIVE,
            "PASSIVE" => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    OK = 0,
    WARNING = 1,
    CRITICAL = 2,
    UNKNOWN = 3,
    PENDING = 4,
}

impl Default for State {
    fn default() -> Self {
        State::OK
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::OK,
            1 => State::WARNING,
            2 => State::CRITICAL,
            3 => State::UNKNOWN,
            4 => State::PENDING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "OK" => State::OK,
            "WARNING" => State::WARNING,
            "CRITICAL" => State::CRITICAL,
            "UNKNOWN" => State::UNKNOWN,
            "PENDING" => State::PENDING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateType {
    SOFT = 0,
    HARD = 1,
}

impl Default for StateType {
    fn default() -> Self {
        StateType::SOFT
    }
}

impl From<i32> for StateType {
    fn from(i: i32) -> Self {
        match i {
            0 => StateType::SOFT,
            1 => StateType::HARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StateType {
    fn from(s: &'a str) -> Self {
        match s {
            "SOFT" => StateType::SOFT,
            "HARD" => StateType::HARD,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ServiceStatus<'a> {
    pub host_id: u64,
    pub service_id: u64,
    pub checked: bool,
    pub check_type: com::centreon::broker::mod_ServiceStatus::CheckType,
    pub state: com::centreon::broker::mod_ServiceStatus::State,
    pub state_type: com::centreon::broker::mod_ServiceStatus::StateType,
    pub last_state_change: i64,
    pub last_hard_state: com::centreon::broker::mod_ServiceStatus::State,
    pub last_hard_state_change: i64,
    pub last_time_ok: i64,
    pub last_time_warning: i64,
    pub last_time_critical: i64,
    pub last_time_unknown: i64,
    pub output: Cow<'a, str>,
    pub long_output: Cow<'a, str>,
    pub perfdata: Cow<'a, str>,
    pub flapping: bool,
    pub percent_state_change: f64,
    pub latency: f64,
    pub execution_time: f64,
    pub last_check: i64,
    pub next_check: i64,
    pub should_be_scheduled: bool,
    pub check_attempt: i32,
    pub notification_number: i32,
    pub no_more_notifications: bool,
    pub last_notification: i64,
    pub next_notification: i64,
    pub acknowledgement_type: com::centreon::broker::AckType,
    pub scheduled_downtime_depth: i32,
    pub type_pb: com::centreon::broker::ServiceType,
    pub internal_id: u64,
}

impl<'a> MessageRead<'a> for ServiceStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.service_id = r.read_uint64(bytes)?,
                Ok(24) => msg.checked = r.read_bool(bytes)?,
                Ok(32) => msg.check_type = r.read_enum(bytes)?,
                Ok(40) => msg.state = r.read_enum(bytes)?,
                Ok(48) => msg.state_type = r.read_enum(bytes)?,
                Ok(56) => msg.last_state_change = r.read_int64(bytes)?,
                Ok(64) => msg.last_hard_state = r.read_enum(bytes)?,
                Ok(72) => msg.last_hard_state_change = r.read_int64(bytes)?,
                Ok(80) => msg.last_time_ok = r.read_int64(bytes)?,
                Ok(88) => msg.last_time_warning = r.read_int64(bytes)?,
                Ok(96) => msg.last_time_critical = r.read_int64(bytes)?,
                Ok(104) => msg.last_time_unknown = r.read_int64(bytes)?,
                Ok(114) => msg.output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.long_output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(130) => msg.perfdata = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(136) => msg.flapping = r.read_bool(bytes)?,
                Ok(145) => msg.percent_state_change = r.read_double(bytes)?,
                Ok(153) => msg.latency = r.read_double(bytes)?,
                Ok(161) => msg.execution_time = r.read_double(bytes)?,
                Ok(168) => msg.last_check = r.read_int64(bytes)?,
                Ok(176) => msg.next_check = r.read_int64(bytes)?,
                Ok(184) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(192) => msg.check_attempt = r.read_int32(bytes)?,
                Ok(200) => msg.notification_number = r.read_int32(bytes)?,
                Ok(208) => msg.no_more_notifications = r.read_bool(bytes)?,
                Ok(216) => msg.last_notification = r.read_int64(bytes)?,
                Ok(224) => msg.next_notification = r.read_int64(bytes)?,
                Ok(232) => msg.acknowledgement_type = r.read_enum(bytes)?,
                Ok(240) => msg.scheduled_downtime_depth = r.read_int32(bytes)?,
                Ok(248) => msg.type_pb = r.read_enum(bytes)?,
                Ok(256) => msg.internal_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServiceStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.checked == false { 0 } else { 1 + sizeof_varint(*(&self.checked) as u64) }
        + if self.check_type == com::centreon::broker::mod_ServiceStatus::CheckType::ACTIVE { 0 } else { 1 + sizeof_varint(*(&self.check_type) as u64) }
        + if self.state == com::centreon::broker::mod_ServiceStatus::State::OK { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + if self.state_type == com::centreon::broker::mod_ServiceStatus::StateType::SOFT { 0 } else { 1 + sizeof_varint(*(&self.state_type) as u64) }
        + if self.last_state_change == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_state_change) as u64) }
        + if self.last_hard_state == com::centreon::broker::mod_ServiceStatus::State::OK { 0 } else { 1 + sizeof_varint(*(&self.last_hard_state) as u64) }
        + if self.last_hard_state_change == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_hard_state_change) as u64) }
        + if self.last_time_ok == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_ok) as u64) }
        + if self.last_time_warning == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_warning) as u64) }
        + if self.last_time_critical == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_critical) as u64) }
        + if self.last_time_unknown == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_unknown) as u64) }
        + if self.output == "" { 0 } else { 1 + sizeof_len((&self.output).len()) }
        + if self.long_output == "" { 0 } else { 1 + sizeof_len((&self.long_output).len()) }
        + if self.perfdata == "" { 0 } else { 2 + sizeof_len((&self.perfdata).len()) }
        + if self.flapping == false { 0 } else { 2 + sizeof_varint(*(&self.flapping) as u64) }
        + if self.percent_state_change == 0f64 { 0 } else { 2 + 8 }
        + if self.latency == 0f64 { 0 } else { 2 + 8 }
        + if self.execution_time == 0f64 { 0 } else { 2 + 8 }
        + if self.last_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_check) as u64) }
        + if self.next_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_check) as u64) }
        + if self.should_be_scheduled == false { 0 } else { 2 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.check_attempt == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.check_attempt) as u64) }
        + if self.notification_number == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.notification_number) as u64) }
        + if self.no_more_notifications == false { 0 } else { 2 + sizeof_varint(*(&self.no_more_notifications) as u64) }
        + if self.last_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_notification) as u64) }
        + if self.next_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_notification) as u64) }
        + if self.acknowledgement_type == com::centreon::broker::AckType::NONE { 0 } else { 2 + sizeof_varint(*(&self.acknowledgement_type) as u64) }
        + if self.scheduled_downtime_depth == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.scheduled_downtime_depth) as u64) }
        + if self.type_pb == com::centreon::broker::ServiceType::SERVICE { 0 } else { 2 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.internal_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.internal_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.service_id))?; }
        if self.checked != false { w.write_with_tag(24, |w| w.write_bool(*&self.checked))?; }
        if self.check_type != com::centreon::broker::mod_ServiceStatus::CheckType::ACTIVE { w.write_with_tag(32, |w| w.write_enum(*&self.check_type as i32))?; }
        if self.state != com::centreon::broker::mod_ServiceStatus::State::OK { w.write_with_tag(40, |w| w.write_enum(*&self.state as i32))?; }
        if self.state_type != com::centreon::broker::mod_ServiceStatus::StateType::SOFT { w.write_with_tag(48, |w| w.write_enum(*&self.state_type as i32))?; }
        if self.last_state_change != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.last_state_change))?; }
        if self.last_hard_state != com::centreon::broker::mod_ServiceStatus::State::OK { w.write_with_tag(64, |w| w.write_enum(*&self.last_hard_state as i32))?; }
        if self.last_hard_state_change != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.last_hard_state_change))?; }
        if self.last_time_ok != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.last_time_ok))?; }
        if self.last_time_warning != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.last_time_warning))?; }
        if self.last_time_critical != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.last_time_critical))?; }
        if self.last_time_unknown != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.last_time_unknown))?; }
        if self.output != "" { w.write_with_tag(114, |w| w.write_string(&**&self.output))?; }
        if self.long_output != "" { w.write_with_tag(122, |w| w.write_string(&**&self.long_output))?; }
        if self.perfdata != "" { w.write_with_tag(130, |w| w.write_string(&**&self.perfdata))?; }
        if self.flapping != false { w.write_with_tag(136, |w| w.write_bool(*&self.flapping))?; }
        if self.percent_state_change != 0f64 { w.write_with_tag(145, |w| w.write_double(*&self.percent_state_change))?; }
        if self.latency != 0f64 { w.write_with_tag(153, |w| w.write_double(*&self.latency))?; }
        if self.execution_time != 0f64 { w.write_with_tag(161, |w| w.write_double(*&self.execution_time))?; }
        if self.last_check != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.last_check))?; }
        if self.next_check != 0i64 { w.write_with_tag(176, |w| w.write_int64(*&self.next_check))?; }
        if self.should_be_scheduled != false { w.write_with_tag(184, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.check_attempt != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.check_attempt))?; }
        if self.notification_number != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.notification_number))?; }
        if self.no_more_notifications != false { w.write_with_tag(208, |w| w.write_bool(*&self.no_more_notifications))?; }
        if self.last_notification != 0i64 { w.write_with_tag(216, |w| w.write_int64(*&self.last_notification))?; }
        if self.next_notification != 0i64 { w.write_with_tag(224, |w| w.write_int64(*&self.next_notification))?; }
        if self.acknowledgement_type != com::centreon::broker::AckType::NONE { w.write_with_tag(232, |w| w.write_enum(*&self.acknowledgement_type as i32))?; }
        if self.scheduled_downtime_depth != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.scheduled_downtime_depth))?; }
        if self.type_pb != com::centreon::broker::ServiceType::SERVICE { w.write_with_tag(248, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.internal_id != 0u64 { w.write_with_tag(256, |w| w.write_uint64(*&self.internal_id))?; }
        Ok(())
    }
}

pub mod mod_ServiceStatus {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CheckType {
    ACTIVE = 0,
    PASSIVE = 1,
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::ACTIVE
    }
}

impl From<i32> for CheckType {
    fn from(i: i32) -> Self {
        match i {
            0 => CheckType::ACTIVE,
            1 => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CheckType {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTIVE" => CheckType::ACTIVE,
            "PASSIVE" => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    OK = 0,
    WARNING = 1,
    CRITICAL = 2,
    UNKNOWN = 3,
    PENDING = 4,
}

impl Default for State {
    fn default() -> Self {
        State::OK
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::OK,
            1 => State::WARNING,
            2 => State::CRITICAL,
            3 => State::UNKNOWN,
            4 => State::PENDING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "OK" => State::OK,
            "WARNING" => State::WARNING,
            "CRITICAL" => State::CRITICAL,
            "UNKNOWN" => State::UNKNOWN,
            "PENDING" => State::PENDING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateType {
    SOFT = 0,
    HARD = 1,
}

impl Default for StateType {
    fn default() -> Self {
        StateType::SOFT
    }
}

impl From<i32> for StateType {
    fn from(i: i32) -> Self {
        match i {
            0 => StateType::SOFT,
            1 => StateType::HARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StateType {
    fn from(s: &'a str) -> Self {
        match s {
            "SOFT" => StateType::SOFT,
            "HARD" => StateType::HARD,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdaptiveService<'a> {
    pub host_id: u64,
    pub service_id: u64,
    pub notify: bool,
    pub active_checks: bool,
    pub should_be_scheduled: bool,
    pub passive_checks: bool,
    pub event_handler_enabled: bool,
    pub flap_detection_enabled: bool,
    pub obsess_over_service: bool,
    pub event_handler: Cow<'a, str>,
    pub check_command: Cow<'a, str>,
    pub check_interval: u32,
    pub retry_interval: u32,
    pub max_check_attempts: u32,
    pub check_freshness: bool,
    pub check_period: Cow<'a, str>,
    pub notification_period: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AdaptiveService<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.service_id = r.read_uint64(bytes)?,
                Ok(24) => msg.notify = r.read_bool(bytes)?,
                Ok(32) => msg.active_checks = r.read_bool(bytes)?,
                Ok(40) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(48) => msg.passive_checks = r.read_bool(bytes)?,
                Ok(56) => msg.event_handler_enabled = r.read_bool(bytes)?,
                Ok(64) => msg.flap_detection_enabled = r.read_bool(bytes)?,
                Ok(72) => msg.obsess_over_service = r.read_bool(bytes)?,
                Ok(82) => msg.event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.check_command = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.check_interval = r.read_uint32(bytes)?,
                Ok(104) => msg.retry_interval = r.read_uint32(bytes)?,
                Ok(112) => msg.max_check_attempts = r.read_uint32(bytes)?,
                Ok(120) => msg.check_freshness = r.read_bool(bytes)?,
                Ok(130) => msg.check_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(138) => msg.notification_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AdaptiveService<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.notify == false { 0 } else { 1 + sizeof_varint(*(&self.notify) as u64) }
        + if self.active_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_checks) as u64) }
        + if self.should_be_scheduled == false { 0 } else { 1 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.passive_checks == false { 0 } else { 1 + sizeof_varint(*(&self.passive_checks) as u64) }
        + if self.event_handler_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.event_handler_enabled) as u64) }
        + if self.flap_detection_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.flap_detection_enabled) as u64) }
        + if self.obsess_over_service == false { 0 } else { 1 + sizeof_varint(*(&self.obsess_over_service) as u64) }
        + if self.event_handler == "" { 0 } else { 1 + sizeof_len((&self.event_handler).len()) }
        + if self.check_command == "" { 0 } else { 1 + sizeof_len((&self.check_command).len()) }
        + if self.check_interval == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.check_interval) as u64) }
        + if self.retry_interval == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.retry_interval) as u64) }
        + if self.max_check_attempts == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.max_check_attempts) as u64) }
        + if self.check_freshness == false { 0 } else { 1 + sizeof_varint(*(&self.check_freshness) as u64) }
        + if self.check_period == "" { 0 } else { 2 + sizeof_len((&self.check_period).len()) }
        + if self.notification_period == "" { 0 } else { 2 + sizeof_len((&self.notification_period).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.service_id))?; }
        if self.notify != false { w.write_with_tag(24, |w| w.write_bool(*&self.notify))?; }
        if self.active_checks != false { w.write_with_tag(32, |w| w.write_bool(*&self.active_checks))?; }
        if self.should_be_scheduled != false { w.write_with_tag(40, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.passive_checks != false { w.write_with_tag(48, |w| w.write_bool(*&self.passive_checks))?; }
        if self.event_handler_enabled != false { w.write_with_tag(56, |w| w.write_bool(*&self.event_handler_enabled))?; }
        if self.flap_detection_enabled != false { w.write_with_tag(64, |w| w.write_bool(*&self.flap_detection_enabled))?; }
        if self.obsess_over_service != false { w.write_with_tag(72, |w| w.write_bool(*&self.obsess_over_service))?; }
        if self.event_handler != "" { w.write_with_tag(82, |w| w.write_string(&**&self.event_handler))?; }
        if self.check_command != "" { w.write_with_tag(90, |w| w.write_string(&**&self.check_command))?; }
        if self.check_interval != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.check_interval))?; }
        if self.retry_interval != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.retry_interval))?; }
        if self.max_check_attempts != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.max_check_attempts))?; }
        if self.check_freshness != false { w.write_with_tag(120, |w| w.write_bool(*&self.check_freshness))?; }
        if self.check_period != "" { w.write_with_tag(130, |w| w.write_string(&**&self.check_period))?; }
        if self.notification_period != "" { w.write_with_tag(138, |w| w.write_string(&**&self.notification_period))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Host<'a> {
    pub host_id: u64,
    pub acknowledged: bool,
    pub acknowledgement_type: com::centreon::broker::AckType,
    pub active_checks: bool,
    pub enabled: bool,
    pub scheduled_downtime_depth: i32,
    pub check_command: Cow<'a, str>,
    pub check_interval: i32,
    pub check_period: Cow<'a, str>,
    pub check_type: com::centreon::broker::mod_Host::CheckType,
    pub check_attempt: i32,
    pub state: com::centreon::broker::mod_Host::State,
    pub event_handler_enabled: bool,
    pub event_handler: Cow<'a, str>,
    pub execution_time: f64,
    pub flap_detection: bool,
    pub checked: bool,
    pub flapping: bool,
    pub last_check: i64,
    pub last_hard_state: com::centreon::broker::mod_Host::State,
    pub last_hard_state_change: i64,
    pub last_notification: i64,
    pub notification_number: i32,
    pub last_state_change: i64,
    pub last_time_down: i64,
    pub last_time_unreachable: i64,
    pub last_time_up: i64,
    pub last_update: i64,
    pub latency: f64,
    pub max_check_attempts: i32,
    pub next_check: i64,
    pub next_host_notification: i64,
    pub no_more_notifications: bool,
    pub notify: bool,
    pub output: Cow<'a, str>,
    pub passive_checks: bool,
    pub percent_state_change: f64,
    pub perfdata: Cow<'a, str>,
    pub retry_interval: f64,
    pub should_be_scheduled: bool,
    pub obsess_over_host: bool,
    pub state_type: com::centreon::broker::mod_Host::StateType,
    pub action_url: Cow<'a, str>,
    pub address: Cow<'a, str>,
    pub alias: Cow<'a, str>,
    pub check_freshness: bool,
    pub default_active_checks: bool,
    pub default_event_handler_enabled: bool,
    pub default_flap_detection: bool,
    pub default_notify: bool,
    pub default_passive_checks: bool,
    pub display_name: Cow<'a, str>,
    pub first_notification_delay: f64,
    pub flap_detection_on_down: bool,
    pub flap_detection_on_unreachable: bool,
    pub flap_detection_on_up: bool,
    pub freshness_threshold: f64,
    pub high_flap_threshold: f64,
    pub name: Cow<'a, str>,
    pub icon_image: Cow<'a, str>,
    pub icon_image_alt: Cow<'a, str>,
    pub instance_id: i32,
    pub low_flap_threshold: f64,
    pub notes: Cow<'a, str>,
    pub notes_url: Cow<'a, str>,
    pub notification_interval: f64,
    pub notification_period: Cow<'a, str>,
    pub notify_on_down: bool,
    pub notify_on_downtime: bool,
    pub notify_on_flapping: bool,
    pub notify_on_recovery: bool,
    pub notify_on_unreachable: bool,
    pub stalk_on_down: bool,
    pub stalk_on_unreachable: bool,
    pub stalk_on_up: bool,
    pub statusmap_image: Cow<'a, str>,
    pub retain_nonstatus_information: bool,
    pub retain_status_information: bool,
    pub timezone: Cow<'a, str>,
    pub severity_id: u64,
    pub tags: Vec<com::centreon::broker::TagInfo>,
    pub icon_id: u64,
}

impl<'a> MessageRead<'a> for Host<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.acknowledged = r.read_bool(bytes)?,
                Ok(24) => msg.acknowledgement_type = r.read_enum(bytes)?,
                Ok(32) => msg.active_checks = r.read_bool(bytes)?,
                Ok(40) => msg.enabled = r.read_bool(bytes)?,
                Ok(48) => msg.scheduled_downtime_depth = r.read_int32(bytes)?,
                Ok(58) => msg.check_command = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.check_interval = r.read_int32(bytes)?,
                Ok(74) => msg.check_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.check_type = r.read_enum(bytes)?,
                Ok(88) => msg.check_attempt = r.read_int32(bytes)?,
                Ok(96) => msg.state = r.read_enum(bytes)?,
                Ok(104) => msg.event_handler_enabled = r.read_bool(bytes)?,
                Ok(114) => msg.event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(121) => msg.execution_time = r.read_double(bytes)?,
                Ok(128) => msg.flap_detection = r.read_bool(bytes)?,
                Ok(136) => msg.checked = r.read_bool(bytes)?,
                Ok(144) => msg.flapping = r.read_bool(bytes)?,
                Ok(152) => msg.last_check = r.read_int64(bytes)?,
                Ok(160) => msg.last_hard_state = r.read_enum(bytes)?,
                Ok(168) => msg.last_hard_state_change = r.read_int64(bytes)?,
                Ok(176) => msg.last_notification = r.read_int64(bytes)?,
                Ok(184) => msg.notification_number = r.read_int32(bytes)?,
                Ok(192) => msg.last_state_change = r.read_int64(bytes)?,
                Ok(200) => msg.last_time_down = r.read_int64(bytes)?,
                Ok(208) => msg.last_time_unreachable = r.read_int64(bytes)?,
                Ok(216) => msg.last_time_up = r.read_int64(bytes)?,
                Ok(224) => msg.last_update = r.read_int64(bytes)?,
                Ok(233) => msg.latency = r.read_double(bytes)?,
                Ok(240) => msg.max_check_attempts = r.read_int32(bytes)?,
                Ok(248) => msg.next_check = r.read_int64(bytes)?,
                Ok(256) => msg.next_host_notification = r.read_int64(bytes)?,
                Ok(264) => msg.no_more_notifications = r.read_bool(bytes)?,
                Ok(272) => msg.notify = r.read_bool(bytes)?,
                Ok(282) => msg.output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(288) => msg.passive_checks = r.read_bool(bytes)?,
                Ok(297) => msg.percent_state_change = r.read_double(bytes)?,
                Ok(306) => msg.perfdata = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(313) => msg.retry_interval = r.read_double(bytes)?,
                Ok(320) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(328) => msg.obsess_over_host = r.read_bool(bytes)?,
                Ok(336) => msg.state_type = r.read_enum(bytes)?,
                Ok(346) => msg.action_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(354) => msg.address = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(362) => msg.alias = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(368) => msg.check_freshness = r.read_bool(bytes)?,
                Ok(376) => msg.default_active_checks = r.read_bool(bytes)?,
                Ok(384) => msg.default_event_handler_enabled = r.read_bool(bytes)?,
                Ok(392) => msg.default_flap_detection = r.read_bool(bytes)?,
                Ok(400) => msg.default_notify = r.read_bool(bytes)?,
                Ok(408) => msg.default_passive_checks = r.read_bool(bytes)?,
                Ok(418) => msg.display_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(425) => msg.first_notification_delay = r.read_double(bytes)?,
                Ok(432) => msg.flap_detection_on_down = r.read_bool(bytes)?,
                Ok(440) => msg.flap_detection_on_unreachable = r.read_bool(bytes)?,
                Ok(448) => msg.flap_detection_on_up = r.read_bool(bytes)?,
                Ok(457) => msg.freshness_threshold = r.read_double(bytes)?,
                Ok(465) => msg.high_flap_threshold = r.read_double(bytes)?,
                Ok(474) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.icon_image = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(490) => msg.icon_image_alt = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(496) => msg.instance_id = r.read_int32(bytes)?,
                Ok(505) => msg.low_flap_threshold = r.read_double(bytes)?,
                Ok(514) => msg.notes = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(522) => msg.notes_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(529) => msg.notification_interval = r.read_double(bytes)?,
                Ok(538) => msg.notification_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(544) => msg.notify_on_down = r.read_bool(bytes)?,
                Ok(552) => msg.notify_on_downtime = r.read_bool(bytes)?,
                Ok(560) => msg.notify_on_flapping = r.read_bool(bytes)?,
                Ok(568) => msg.notify_on_recovery = r.read_bool(bytes)?,
                Ok(576) => msg.notify_on_unreachable = r.read_bool(bytes)?,
                Ok(584) => msg.stalk_on_down = r.read_bool(bytes)?,
                Ok(592) => msg.stalk_on_unreachable = r.read_bool(bytes)?,
                Ok(600) => msg.stalk_on_up = r.read_bool(bytes)?,
                Ok(610) => msg.statusmap_image = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(616) => msg.retain_nonstatus_information = r.read_bool(bytes)?,
                Ok(624) => msg.retain_status_information = r.read_bool(bytes)?,
                Ok(634) => msg.timezone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.severity_id = r.read_uint64(bytes)?,
                Ok(650) => msg.tags.push(r.read_message::<com::centreon::broker::TagInfo>(bytes)?),
                Ok(656) => msg.icon_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Host<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.acknowledged == false { 0 } else { 1 + sizeof_varint(*(&self.acknowledged) as u64) }
        + if self.acknowledgement_type == com::centreon::broker::AckType::NONE { 0 } else { 1 + sizeof_varint(*(&self.acknowledgement_type) as u64) }
        + if self.active_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_checks) as u64) }
        + if self.enabled == false { 0 } else { 1 + sizeof_varint(*(&self.enabled) as u64) }
        + if self.scheduled_downtime_depth == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.scheduled_downtime_depth) as u64) }
        + if self.check_command == "" { 0 } else { 1 + sizeof_len((&self.check_command).len()) }
        + if self.check_interval == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.check_interval) as u64) }
        + if self.check_period == "" { 0 } else { 1 + sizeof_len((&self.check_period).len()) }
        + if self.check_type == com::centreon::broker::mod_Host::CheckType::ACTIVE { 0 } else { 1 + sizeof_varint(*(&self.check_type) as u64) }
        + if self.check_attempt == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.check_attempt) as u64) }
        + if self.state == com::centreon::broker::mod_Host::State::UP { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + if self.event_handler_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.event_handler_enabled) as u64) }
        + if self.event_handler == "" { 0 } else { 1 + sizeof_len((&self.event_handler).len()) }
        + if self.execution_time == 0f64 { 0 } else { 1 + 8 }
        + if self.flap_detection == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection) as u64) }
        + if self.checked == false { 0 } else { 2 + sizeof_varint(*(&self.checked) as u64) }
        + if self.flapping == false { 0 } else { 2 + sizeof_varint(*(&self.flapping) as u64) }
        + if self.last_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_check) as u64) }
        + if self.last_hard_state == com::centreon::broker::mod_Host::State::UP { 0 } else { 2 + sizeof_varint(*(&self.last_hard_state) as u64) }
        + if self.last_hard_state_change == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_hard_state_change) as u64) }
        + if self.last_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_notification) as u64) }
        + if self.notification_number == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.notification_number) as u64) }
        + if self.last_state_change == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_state_change) as u64) }
        + if self.last_time_down == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_down) as u64) }
        + if self.last_time_unreachable == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_unreachable) as u64) }
        + if self.last_time_up == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_time_up) as u64) }
        + if self.last_update == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_update) as u64) }
        + if self.latency == 0f64 { 0 } else { 2 + 8 }
        + if self.max_check_attempts == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.max_check_attempts) as u64) }
        + if self.next_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_check) as u64) }
        + if self.next_host_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_host_notification) as u64) }
        + if self.no_more_notifications == false { 0 } else { 2 + sizeof_varint(*(&self.no_more_notifications) as u64) }
        + if self.notify == false { 0 } else { 2 + sizeof_varint(*(&self.notify) as u64) }
        + if self.output == "" { 0 } else { 2 + sizeof_len((&self.output).len()) }
        + if self.passive_checks == false { 0 } else { 2 + sizeof_varint(*(&self.passive_checks) as u64) }
        + if self.percent_state_change == 0f64 { 0 } else { 2 + 8 }
        + if self.perfdata == "" { 0 } else { 2 + sizeof_len((&self.perfdata).len()) }
        + if self.retry_interval == 0f64 { 0 } else { 2 + 8 }
        + if self.should_be_scheduled == false { 0 } else { 2 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.obsess_over_host == false { 0 } else { 2 + sizeof_varint(*(&self.obsess_over_host) as u64) }
        + if self.state_type == com::centreon::broker::mod_Host::StateType::SOFT { 0 } else { 2 + sizeof_varint(*(&self.state_type) as u64) }
        + if self.action_url == "" { 0 } else { 2 + sizeof_len((&self.action_url).len()) }
        + if self.address == "" { 0 } else { 2 + sizeof_len((&self.address).len()) }
        + if self.alias == "" { 0 } else { 2 + sizeof_len((&self.alias).len()) }
        + if self.check_freshness == false { 0 } else { 2 + sizeof_varint(*(&self.check_freshness) as u64) }
        + if self.default_active_checks == false { 0 } else { 2 + sizeof_varint(*(&self.default_active_checks) as u64) }
        + if self.default_event_handler_enabled == false { 0 } else { 2 + sizeof_varint(*(&self.default_event_handler_enabled) as u64) }
        + if self.default_flap_detection == false { 0 } else { 2 + sizeof_varint(*(&self.default_flap_detection) as u64) }
        + if self.default_notify == false { 0 } else { 2 + sizeof_varint(*(&self.default_notify) as u64) }
        + if self.default_passive_checks == false { 0 } else { 2 + sizeof_varint(*(&self.default_passive_checks) as u64) }
        + if self.display_name == "" { 0 } else { 2 + sizeof_len((&self.display_name).len()) }
        + if self.first_notification_delay == 0f64 { 0 } else { 2 + 8 }
        + if self.flap_detection_on_down == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_down) as u64) }
        + if self.flap_detection_on_unreachable == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_unreachable) as u64) }
        + if self.flap_detection_on_up == false { 0 } else { 2 + sizeof_varint(*(&self.flap_detection_on_up) as u64) }
        + if self.freshness_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.high_flap_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.name == "" { 0 } else { 2 + sizeof_len((&self.name).len()) }
        + if self.icon_image == "" { 0 } else { 2 + sizeof_len((&self.icon_image).len()) }
        + if self.icon_image_alt == "" { 0 } else { 2 + sizeof_len((&self.icon_image_alt).len()) }
        + if self.instance_id == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.instance_id) as u64) }
        + if self.low_flap_threshold == 0f64 { 0 } else { 2 + 8 }
        + if self.notes == "" { 0 } else { 2 + sizeof_len((&self.notes).len()) }
        + if self.notes_url == "" { 0 } else { 2 + sizeof_len((&self.notes_url).len()) }
        + if self.notification_interval == 0f64 { 0 } else { 2 + 8 }
        + if self.notification_period == "" { 0 } else { 2 + sizeof_len((&self.notification_period).len()) }
        + if self.notify_on_down == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_down) as u64) }
        + if self.notify_on_downtime == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_downtime) as u64) }
        + if self.notify_on_flapping == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_flapping) as u64) }
        + if self.notify_on_recovery == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_recovery) as u64) }
        + if self.notify_on_unreachable == false { 0 } else { 2 + sizeof_varint(*(&self.notify_on_unreachable) as u64) }
        + if self.stalk_on_down == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_down) as u64) }
        + if self.stalk_on_unreachable == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_unreachable) as u64) }
        + if self.stalk_on_up == false { 0 } else { 2 + sizeof_varint(*(&self.stalk_on_up) as u64) }
        + if self.statusmap_image == "" { 0 } else { 2 + sizeof_len((&self.statusmap_image).len()) }
        + if self.retain_nonstatus_information == false { 0 } else { 2 + sizeof_varint(*(&self.retain_nonstatus_information) as u64) }
        + if self.retain_status_information == false { 0 } else { 2 + sizeof_varint(*(&self.retain_status_information) as u64) }
        + if self.timezone == "" { 0 } else { 2 + sizeof_len((&self.timezone).len()) }
        + if self.severity_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.severity_id) as u64) }
        + self.tags.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.icon_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.icon_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.acknowledged != false { w.write_with_tag(16, |w| w.write_bool(*&self.acknowledged))?; }
        if self.acknowledgement_type != com::centreon::broker::AckType::NONE { w.write_with_tag(24, |w| w.write_enum(*&self.acknowledgement_type as i32))?; }
        if self.active_checks != false { w.write_with_tag(32, |w| w.write_bool(*&self.active_checks))?; }
        if self.enabled != false { w.write_with_tag(40, |w| w.write_bool(*&self.enabled))?; }
        if self.scheduled_downtime_depth != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.scheduled_downtime_depth))?; }
        if self.check_command != "" { w.write_with_tag(58, |w| w.write_string(&**&self.check_command))?; }
        if self.check_interval != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.check_interval))?; }
        if self.check_period != "" { w.write_with_tag(74, |w| w.write_string(&**&self.check_period))?; }
        if self.check_type != com::centreon::broker::mod_Host::CheckType::ACTIVE { w.write_with_tag(80, |w| w.write_enum(*&self.check_type as i32))?; }
        if self.check_attempt != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.check_attempt))?; }
        if self.state != com::centreon::broker::mod_Host::State::UP { w.write_with_tag(96, |w| w.write_enum(*&self.state as i32))?; }
        if self.event_handler_enabled != false { w.write_with_tag(104, |w| w.write_bool(*&self.event_handler_enabled))?; }
        if self.event_handler != "" { w.write_with_tag(114, |w| w.write_string(&**&self.event_handler))?; }
        if self.execution_time != 0f64 { w.write_with_tag(121, |w| w.write_double(*&self.execution_time))?; }
        if self.flap_detection != false { w.write_with_tag(128, |w| w.write_bool(*&self.flap_detection))?; }
        if self.checked != false { w.write_with_tag(136, |w| w.write_bool(*&self.checked))?; }
        if self.flapping != false { w.write_with_tag(144, |w| w.write_bool(*&self.flapping))?; }
        if self.last_check != 0i64 { w.write_with_tag(152, |w| w.write_int64(*&self.last_check))?; }
        if self.last_hard_state != com::centreon::broker::mod_Host::State::UP { w.write_with_tag(160, |w| w.write_enum(*&self.last_hard_state as i32))?; }
        if self.last_hard_state_change != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.last_hard_state_change))?; }
        if self.last_notification != 0i64 { w.write_with_tag(176, |w| w.write_int64(*&self.last_notification))?; }
        if self.notification_number != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.notification_number))?; }
        if self.last_state_change != 0i64 { w.write_with_tag(192, |w| w.write_int64(*&self.last_state_change))?; }
        if self.last_time_down != 0i64 { w.write_with_tag(200, |w| w.write_int64(*&self.last_time_down))?; }
        if self.last_time_unreachable != 0i64 { w.write_with_tag(208, |w| w.write_int64(*&self.last_time_unreachable))?; }
        if self.last_time_up != 0i64 { w.write_with_tag(216, |w| w.write_int64(*&self.last_time_up))?; }
        if self.last_update != 0i64 { w.write_with_tag(224, |w| w.write_int64(*&self.last_update))?; }
        if self.latency != 0f64 { w.write_with_tag(233, |w| w.write_double(*&self.latency))?; }
        if self.max_check_attempts != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.max_check_attempts))?; }
        if self.next_check != 0i64 { w.write_with_tag(248, |w| w.write_int64(*&self.next_check))?; }
        if self.next_host_notification != 0i64 { w.write_with_tag(256, |w| w.write_int64(*&self.next_host_notification))?; }
        if self.no_more_notifications != false { w.write_with_tag(264, |w| w.write_bool(*&self.no_more_notifications))?; }
        if self.notify != false { w.write_with_tag(272, |w| w.write_bool(*&self.notify))?; }
        if self.output != "" { w.write_with_tag(282, |w| w.write_string(&**&self.output))?; }
        if self.passive_checks != false { w.write_with_tag(288, |w| w.write_bool(*&self.passive_checks))?; }
        if self.percent_state_change != 0f64 { w.write_with_tag(297, |w| w.write_double(*&self.percent_state_change))?; }
        if self.perfdata != "" { w.write_with_tag(306, |w| w.write_string(&**&self.perfdata))?; }
        if self.retry_interval != 0f64 { w.write_with_tag(313, |w| w.write_double(*&self.retry_interval))?; }
        if self.should_be_scheduled != false { w.write_with_tag(320, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.obsess_over_host != false { w.write_with_tag(328, |w| w.write_bool(*&self.obsess_over_host))?; }
        if self.state_type != com::centreon::broker::mod_Host::StateType::SOFT { w.write_with_tag(336, |w| w.write_enum(*&self.state_type as i32))?; }
        if self.action_url != "" { w.write_with_tag(346, |w| w.write_string(&**&self.action_url))?; }
        if self.address != "" { w.write_with_tag(354, |w| w.write_string(&**&self.address))?; }
        if self.alias != "" { w.write_with_tag(362, |w| w.write_string(&**&self.alias))?; }
        if self.check_freshness != false { w.write_with_tag(368, |w| w.write_bool(*&self.check_freshness))?; }
        if self.default_active_checks != false { w.write_with_tag(376, |w| w.write_bool(*&self.default_active_checks))?; }
        if self.default_event_handler_enabled != false { w.write_with_tag(384, |w| w.write_bool(*&self.default_event_handler_enabled))?; }
        if self.default_flap_detection != false { w.write_with_tag(392, |w| w.write_bool(*&self.default_flap_detection))?; }
        if self.default_notify != false { w.write_with_tag(400, |w| w.write_bool(*&self.default_notify))?; }
        if self.default_passive_checks != false { w.write_with_tag(408, |w| w.write_bool(*&self.default_passive_checks))?; }
        if self.display_name != "" { w.write_with_tag(418, |w| w.write_string(&**&self.display_name))?; }
        if self.first_notification_delay != 0f64 { w.write_with_tag(425, |w| w.write_double(*&self.first_notification_delay))?; }
        if self.flap_detection_on_down != false { w.write_with_tag(432, |w| w.write_bool(*&self.flap_detection_on_down))?; }
        if self.flap_detection_on_unreachable != false { w.write_with_tag(440, |w| w.write_bool(*&self.flap_detection_on_unreachable))?; }
        if self.flap_detection_on_up != false { w.write_with_tag(448, |w| w.write_bool(*&self.flap_detection_on_up))?; }
        if self.freshness_threshold != 0f64 { w.write_with_tag(457, |w| w.write_double(*&self.freshness_threshold))?; }
        if self.high_flap_threshold != 0f64 { w.write_with_tag(465, |w| w.write_double(*&self.high_flap_threshold))?; }
        if self.name != "" { w.write_with_tag(474, |w| w.write_string(&**&self.name))?; }
        if self.icon_image != "" { w.write_with_tag(482, |w| w.write_string(&**&self.icon_image))?; }
        if self.icon_image_alt != "" { w.write_with_tag(490, |w| w.write_string(&**&self.icon_image_alt))?; }
        if self.instance_id != 0i32 { w.write_with_tag(496, |w| w.write_int32(*&self.instance_id))?; }
        if self.low_flap_threshold != 0f64 { w.write_with_tag(505, |w| w.write_double(*&self.low_flap_threshold))?; }
        if self.notes != "" { w.write_with_tag(514, |w| w.write_string(&**&self.notes))?; }
        if self.notes_url != "" { w.write_with_tag(522, |w| w.write_string(&**&self.notes_url))?; }
        if self.notification_interval != 0f64 { w.write_with_tag(529, |w| w.write_double(*&self.notification_interval))?; }
        if self.notification_period != "" { w.write_with_tag(538, |w| w.write_string(&**&self.notification_period))?; }
        if self.notify_on_down != false { w.write_with_tag(544, |w| w.write_bool(*&self.notify_on_down))?; }
        if self.notify_on_downtime != false { w.write_with_tag(552, |w| w.write_bool(*&self.notify_on_downtime))?; }
        if self.notify_on_flapping != false { w.write_with_tag(560, |w| w.write_bool(*&self.notify_on_flapping))?; }
        if self.notify_on_recovery != false { w.write_with_tag(568, |w| w.write_bool(*&self.notify_on_recovery))?; }
        if self.notify_on_unreachable != false { w.write_with_tag(576, |w| w.write_bool(*&self.notify_on_unreachable))?; }
        if self.stalk_on_down != false { w.write_with_tag(584, |w| w.write_bool(*&self.stalk_on_down))?; }
        if self.stalk_on_unreachable != false { w.write_with_tag(592, |w| w.write_bool(*&self.stalk_on_unreachable))?; }
        if self.stalk_on_up != false { w.write_with_tag(600, |w| w.write_bool(*&self.stalk_on_up))?; }
        if self.statusmap_image != "" { w.write_with_tag(610, |w| w.write_string(&**&self.statusmap_image))?; }
        if self.retain_nonstatus_information != false { w.write_with_tag(616, |w| w.write_bool(*&self.retain_nonstatus_information))?; }
        if self.retain_status_information != false { w.write_with_tag(624, |w| w.write_bool(*&self.retain_status_information))?; }
        if self.timezone != "" { w.write_with_tag(634, |w| w.write_string(&**&self.timezone))?; }
        if self.severity_id != 0u64 { w.write_with_tag(640, |w| w.write_uint64(*&self.severity_id))?; }
        for s in &self.tags { w.write_with_tag(650, |w| w.write_message(s))?; }
        if self.icon_id != 0u64 { w.write_with_tag(656, |w| w.write_uint64(*&self.icon_id))?; }
        Ok(())
    }
}

pub mod mod_Host {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CheckType {
    ACTIVE = 0,
    PASSIVE = 1,
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::ACTIVE
    }
}

impl From<i32> for CheckType {
    fn from(i: i32) -> Self {
        match i {
            0 => CheckType::ACTIVE,
            1 => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CheckType {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTIVE" => CheckType::ACTIVE,
            "PASSIVE" => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    UP = 0,
    DOWN = 1,
    UNREACHABLE = 2,
}

impl Default for State {
    fn default() -> Self {
        State::UP
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::UP,
            1 => State::DOWN,
            2 => State::UNREACHABLE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "UP" => State::UP,
            "DOWN" => State::DOWN,
            "UNREACHABLE" => State::UNREACHABLE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateType {
    SOFT = 0,
    HARD = 1,
}

impl Default for StateType {
    fn default() -> Self {
        StateType::SOFT
    }
}

impl From<i32> for StateType {
    fn from(i: i32) -> Self {
        match i {
            0 => StateType::SOFT,
            1 => StateType::HARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StateType {
    fn from(s: &'a str) -> Self {
        match s {
            "SOFT" => StateType::SOFT,
            "HARD" => StateType::HARD,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HostStatus<'a> {
    pub host_id: u64,
    pub checked: bool,
    pub check_type: com::centreon::broker::mod_HostStatus::CheckType,
    pub state: com::centreon::broker::mod_HostStatus::State,
    pub state_type: com::centreon::broker::mod_HostStatus::StateType,
    pub last_state_change: i64,
    pub last_hard_state: com::centreon::broker::mod_HostStatus::State,
    pub last_hard_state_change: i64,
    pub last_time_up: i64,
    pub last_time_down: i64,
    pub last_time_unreachable: i64,
    pub output: Cow<'a, str>,
    pub long_output: Cow<'a, str>,
    pub perfdata: Cow<'a, str>,
    pub flapping: bool,
    pub percent_state_change: f64,
    pub latency: f64,
    pub execution_time: f64,
    pub last_check: i64,
    pub next_check: i64,
    pub should_be_scheduled: bool,
    pub check_attempt: i32,
    pub notification_number: i32,
    pub no_more_notifications: bool,
    pub last_notification: i64,
    pub next_host_notification: i64,
    pub acknowledgement_type: com::centreon::broker::AckType,
    pub scheduled_downtime_depth: i32,
}

impl<'a> MessageRead<'a> for HostStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.checked = r.read_bool(bytes)?,
                Ok(24) => msg.check_type = r.read_enum(bytes)?,
                Ok(32) => msg.state = r.read_enum(bytes)?,
                Ok(40) => msg.state_type = r.read_enum(bytes)?,
                Ok(48) => msg.last_state_change = r.read_int64(bytes)?,
                Ok(56) => msg.last_hard_state = r.read_enum(bytes)?,
                Ok(64) => msg.last_hard_state_change = r.read_int64(bytes)?,
                Ok(72) => msg.last_time_up = r.read_int64(bytes)?,
                Ok(80) => msg.last_time_down = r.read_int64(bytes)?,
                Ok(88) => msg.last_time_unreachable = r.read_int64(bytes)?,
                Ok(98) => msg.output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(106) => msg.long_output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.perfdata = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(120) => msg.flapping = r.read_bool(bytes)?,
                Ok(129) => msg.percent_state_change = r.read_double(bytes)?,
                Ok(137) => msg.latency = r.read_double(bytes)?,
                Ok(145) => msg.execution_time = r.read_double(bytes)?,
                Ok(152) => msg.last_check = r.read_int64(bytes)?,
                Ok(160) => msg.next_check = r.read_int64(bytes)?,
                Ok(168) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(176) => msg.check_attempt = r.read_int32(bytes)?,
                Ok(184) => msg.notification_number = r.read_int32(bytes)?,
                Ok(192) => msg.no_more_notifications = r.read_bool(bytes)?,
                Ok(200) => msg.last_notification = r.read_int64(bytes)?,
                Ok(208) => msg.next_host_notification = r.read_int64(bytes)?,
                Ok(216) => msg.acknowledgement_type = r.read_enum(bytes)?,
                Ok(224) => msg.scheduled_downtime_depth = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HostStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.checked == false { 0 } else { 1 + sizeof_varint(*(&self.checked) as u64) }
        + if self.check_type == com::centreon::broker::mod_HostStatus::CheckType::ACTIVE { 0 } else { 1 + sizeof_varint(*(&self.check_type) as u64) }
        + if self.state == com::centreon::broker::mod_HostStatus::State::UP { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + if self.state_type == com::centreon::broker::mod_HostStatus::StateType::SOFT { 0 } else { 1 + sizeof_varint(*(&self.state_type) as u64) }
        + if self.last_state_change == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_state_change) as u64) }
        + if self.last_hard_state == com::centreon::broker::mod_HostStatus::State::UP { 0 } else { 1 + sizeof_varint(*(&self.last_hard_state) as u64) }
        + if self.last_hard_state_change == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_hard_state_change) as u64) }
        + if self.last_time_up == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_up) as u64) }
        + if self.last_time_down == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_down) as u64) }
        + if self.last_time_unreachable == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_time_unreachable) as u64) }
        + if self.output == "" { 0 } else { 1 + sizeof_len((&self.output).len()) }
        + if self.long_output == "" { 0 } else { 1 + sizeof_len((&self.long_output).len()) }
        + if self.perfdata == "" { 0 } else { 1 + sizeof_len((&self.perfdata).len()) }
        + if self.flapping == false { 0 } else { 1 + sizeof_varint(*(&self.flapping) as u64) }
        + if self.percent_state_change == 0f64 { 0 } else { 2 + 8 }
        + if self.latency == 0f64 { 0 } else { 2 + 8 }
        + if self.execution_time == 0f64 { 0 } else { 2 + 8 }
        + if self.last_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_check) as u64) }
        + if self.next_check == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_check) as u64) }
        + if self.should_be_scheduled == false { 0 } else { 2 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.check_attempt == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.check_attempt) as u64) }
        + if self.notification_number == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.notification_number) as u64) }
        + if self.no_more_notifications == false { 0 } else { 2 + sizeof_varint(*(&self.no_more_notifications) as u64) }
        + if self.last_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.last_notification) as u64) }
        + if self.next_host_notification == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.next_host_notification) as u64) }
        + if self.acknowledgement_type == com::centreon::broker::AckType::NONE { 0 } else { 2 + sizeof_varint(*(&self.acknowledgement_type) as u64) }
        + if self.scheduled_downtime_depth == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.scheduled_downtime_depth) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.checked != false { w.write_with_tag(16, |w| w.write_bool(*&self.checked))?; }
        if self.check_type != com::centreon::broker::mod_HostStatus::CheckType::ACTIVE { w.write_with_tag(24, |w| w.write_enum(*&self.check_type as i32))?; }
        if self.state != com::centreon::broker::mod_HostStatus::State::UP { w.write_with_tag(32, |w| w.write_enum(*&self.state as i32))?; }
        if self.state_type != com::centreon::broker::mod_HostStatus::StateType::SOFT { w.write_with_tag(40, |w| w.write_enum(*&self.state_type as i32))?; }
        if self.last_state_change != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.last_state_change))?; }
        if self.last_hard_state != com::centreon::broker::mod_HostStatus::State::UP { w.write_with_tag(56, |w| w.write_enum(*&self.last_hard_state as i32))?; }
        if self.last_hard_state_change != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.last_hard_state_change))?; }
        if self.last_time_up != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.last_time_up))?; }
        if self.last_time_down != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.last_time_down))?; }
        if self.last_time_unreachable != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.last_time_unreachable))?; }
        if self.output != "" { w.write_with_tag(98, |w| w.write_string(&**&self.output))?; }
        if self.long_output != "" { w.write_with_tag(106, |w| w.write_string(&**&self.long_output))?; }
        if self.perfdata != "" { w.write_with_tag(114, |w| w.write_string(&**&self.perfdata))?; }
        if self.flapping != false { w.write_with_tag(120, |w| w.write_bool(*&self.flapping))?; }
        if self.percent_state_change != 0f64 { w.write_with_tag(129, |w| w.write_double(*&self.percent_state_change))?; }
        if self.latency != 0f64 { w.write_with_tag(137, |w| w.write_double(*&self.latency))?; }
        if self.execution_time != 0f64 { w.write_with_tag(145, |w| w.write_double(*&self.execution_time))?; }
        if self.last_check != 0i64 { w.write_with_tag(152, |w| w.write_int64(*&self.last_check))?; }
        if self.next_check != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.next_check))?; }
        if self.should_be_scheduled != false { w.write_with_tag(168, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.check_attempt != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.check_attempt))?; }
        if self.notification_number != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.notification_number))?; }
        if self.no_more_notifications != false { w.write_with_tag(192, |w| w.write_bool(*&self.no_more_notifications))?; }
        if self.last_notification != 0i64 { w.write_with_tag(200, |w| w.write_int64(*&self.last_notification))?; }
        if self.next_host_notification != 0i64 { w.write_with_tag(208, |w| w.write_int64(*&self.next_host_notification))?; }
        if self.acknowledgement_type != com::centreon::broker::AckType::NONE { w.write_with_tag(216, |w| w.write_enum(*&self.acknowledgement_type as i32))?; }
        if self.scheduled_downtime_depth != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.scheduled_downtime_depth))?; }
        Ok(())
    }
}

pub mod mod_HostStatus {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CheckType {
    ACTIVE = 0,
    PASSIVE = 1,
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::ACTIVE
    }
}

impl From<i32> for CheckType {
    fn from(i: i32) -> Self {
        match i {
            0 => CheckType::ACTIVE,
            1 => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CheckType {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTIVE" => CheckType::ACTIVE,
            "PASSIVE" => CheckType::PASSIVE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    UP = 0,
    DOWN = 1,
    UNREACHABLE = 2,
}

impl Default for State {
    fn default() -> Self {
        State::UP
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::UP,
            1 => State::DOWN,
            2 => State::UNREACHABLE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "UP" => State::UP,
            "DOWN" => State::DOWN,
            "UNREACHABLE" => State::UNREACHABLE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateType {
    SOFT = 0,
    HARD = 1,
}

impl Default for StateType {
    fn default() -> Self {
        StateType::SOFT
    }
}

impl From<i32> for StateType {
    fn from(i: i32) -> Self {
        match i {
            0 => StateType::SOFT,
            1 => StateType::HARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StateType {
    fn from(s: &'a str) -> Self {
        match s {
            "SOFT" => StateType::SOFT,
            "HARD" => StateType::HARD,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdaptiveHost<'a> {
    pub host_id: u64,
    pub notify: bool,
    pub active_checks: bool,
    pub should_be_scheduled: bool,
    pub passive_checks: bool,
    pub event_handler_enabled: bool,
    pub flap_detection: bool,
    pub obsess_over_host: bool,
    pub event_handler: Cow<'a, str>,
    pub check_command: Cow<'a, str>,
    pub check_interval: u32,
    pub retry_interval: u32,
    pub max_check_attempts: u32,
    pub check_freshness: bool,
    pub check_period: Cow<'a, str>,
    pub notification_period: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AdaptiveHost<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.notify = r.read_bool(bytes)?,
                Ok(24) => msg.active_checks = r.read_bool(bytes)?,
                Ok(32) => msg.should_be_scheduled = r.read_bool(bytes)?,
                Ok(40) => msg.passive_checks = r.read_bool(bytes)?,
                Ok(48) => msg.event_handler_enabled = r.read_bool(bytes)?,
                Ok(56) => msg.flap_detection = r.read_bool(bytes)?,
                Ok(64) => msg.obsess_over_host = r.read_bool(bytes)?,
                Ok(74) => msg.event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.check_command = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.check_interval = r.read_uint32(bytes)?,
                Ok(96) => msg.retry_interval = r.read_uint32(bytes)?,
                Ok(104) => msg.max_check_attempts = r.read_uint32(bytes)?,
                Ok(112) => msg.check_freshness = r.read_bool(bytes)?,
                Ok(122) => msg.check_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(130) => msg.notification_period = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AdaptiveHost<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.notify == false { 0 } else { 1 + sizeof_varint(*(&self.notify) as u64) }
        + if self.active_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_checks) as u64) }
        + if self.should_be_scheduled == false { 0 } else { 1 + sizeof_varint(*(&self.should_be_scheduled) as u64) }
        + if self.passive_checks == false { 0 } else { 1 + sizeof_varint(*(&self.passive_checks) as u64) }
        + if self.event_handler_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.event_handler_enabled) as u64) }
        + if self.flap_detection == false { 0 } else { 1 + sizeof_varint(*(&self.flap_detection) as u64) }
        + if self.obsess_over_host == false { 0 } else { 1 + sizeof_varint(*(&self.obsess_over_host) as u64) }
        + if self.event_handler == "" { 0 } else { 1 + sizeof_len((&self.event_handler).len()) }
        + if self.check_command == "" { 0 } else { 1 + sizeof_len((&self.check_command).len()) }
        + if self.check_interval == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.check_interval) as u64) }
        + if self.retry_interval == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.retry_interval) as u64) }
        + if self.max_check_attempts == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.max_check_attempts) as u64) }
        + if self.check_freshness == false { 0 } else { 1 + sizeof_varint(*(&self.check_freshness) as u64) }
        + if self.check_period == "" { 0 } else { 1 + sizeof_len((&self.check_period).len()) }
        + if self.notification_period == "" { 0 } else { 2 + sizeof_len((&self.notification_period).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.notify != false { w.write_with_tag(16, |w| w.write_bool(*&self.notify))?; }
        if self.active_checks != false { w.write_with_tag(24, |w| w.write_bool(*&self.active_checks))?; }
        if self.should_be_scheduled != false { w.write_with_tag(32, |w| w.write_bool(*&self.should_be_scheduled))?; }
        if self.passive_checks != false { w.write_with_tag(40, |w| w.write_bool(*&self.passive_checks))?; }
        if self.event_handler_enabled != false { w.write_with_tag(48, |w| w.write_bool(*&self.event_handler_enabled))?; }
        if self.flap_detection != false { w.write_with_tag(56, |w| w.write_bool(*&self.flap_detection))?; }
        if self.obsess_over_host != false { w.write_with_tag(64, |w| w.write_bool(*&self.obsess_over_host))?; }
        if self.event_handler != "" { w.write_with_tag(74, |w| w.write_string(&**&self.event_handler))?; }
        if self.check_command != "" { w.write_with_tag(82, |w| w.write_string(&**&self.check_command))?; }
        if self.check_interval != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.check_interval))?; }
        if self.retry_interval != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.retry_interval))?; }
        if self.max_check_attempts != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.max_check_attempts))?; }
        if self.check_freshness != false { w.write_with_tag(112, |w| w.write_bool(*&self.check_freshness))?; }
        if self.check_period != "" { w.write_with_tag(122, |w| w.write_string(&**&self.check_period))?; }
        if self.notification_period != "" { w.write_with_tag(130, |w| w.write_string(&**&self.notification_period))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Comment<'a> {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub author: Cow<'a, str>,
    pub type_pb: com::centreon::broker::mod_Comment::Type,
    pub data: Cow<'a, str>,
    pub deletion_time: u64,
    pub entry_time: u64,
    pub entry_type: com::centreon::broker::mod_Comment::EntryType,
    pub expire_time: u64,
    pub expires: bool,
    pub host_id: u64,
    pub internal_id: u64,
    pub persistent: bool,
    pub instance_id: u64,
    pub service_id: u64,
    pub source: com::centreon::broker::mod_Comment::Src,
}

impl<'a> MessageRead<'a> for Comment<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(18) => msg.author = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(34) => msg.data = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.deletion_time = r.read_uint64(bytes)?,
                Ok(48) => msg.entry_time = r.read_uint64(bytes)?,
                Ok(56) => msg.entry_type = r.read_enum(bytes)?,
                Ok(64) => msg.expire_time = r.read_uint64(bytes)?,
                Ok(72) => msg.expires = r.read_bool(bytes)?,
                Ok(80) => msg.host_id = r.read_uint64(bytes)?,
                Ok(88) => msg.internal_id = r.read_uint64(bytes)?,
                Ok(96) => msg.persistent = r.read_bool(bytes)?,
                Ok(104) => msg.instance_id = r.read_uint64(bytes)?,
                Ok(112) => msg.service_id = r.read_uint64(bytes)?,
                Ok(120) => msg.source = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Comment<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.author == "" { 0 } else { 1 + sizeof_len((&self.author).len()) }
        + if self.type_pb == com::centreon::broker::mod_Comment::Type::NO_TYPE { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.data == "" { 0 } else { 1 + sizeof_len((&self.data).len()) }
        + if self.deletion_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.deletion_time) as u64) }
        + if self.entry_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.entry_time) as u64) }
        + if self.entry_type == com::centreon::broker::mod_Comment::EntryType::NO_ENTRY_TYPE { 0 } else { 1 + sizeof_varint(*(&self.entry_type) as u64) }
        + if self.expire_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expire_time) as u64) }
        + if self.expires == false { 0 } else { 1 + sizeof_varint(*(&self.expires) as u64) }
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.internal_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.internal_id) as u64) }
        + if self.persistent == false { 0 } else { 1 + sizeof_varint(*(&self.persistent) as u64) }
        + if self.instance_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.instance_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.source == com::centreon::broker::mod_Comment::Src::INTERNAL { 0 } else { 1 + sizeof_varint(*(&self.source) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.author != "" { w.write_with_tag(18, |w| w.write_string(&**&self.author))?; }
        if self.type_pb != com::centreon::broker::mod_Comment::Type::NO_TYPE { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.data != "" { w.write_with_tag(34, |w| w.write_string(&**&self.data))?; }
        if self.deletion_time != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.deletion_time))?; }
        if self.entry_time != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.entry_time))?; }
        if self.entry_type != com::centreon::broker::mod_Comment::EntryType::NO_ENTRY_TYPE { w.write_with_tag(56, |w| w.write_enum(*&self.entry_type as i32))?; }
        if self.expire_time != 0u64 { w.write_with_tag(64, |w| w.write_uint64(*&self.expire_time))?; }
        if self.expires != false { w.write_with_tag(72, |w| w.write_bool(*&self.expires))?; }
        if self.host_id != 0u64 { w.write_with_tag(80, |w| w.write_uint64(*&self.host_id))?; }
        if self.internal_id != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.internal_id))?; }
        if self.persistent != false { w.write_with_tag(96, |w| w.write_bool(*&self.persistent))?; }
        if self.instance_id != 0u64 { w.write_with_tag(104, |w| w.write_uint64(*&self.instance_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(112, |w| w.write_uint64(*&self.service_id))?; }
        if self.source != com::centreon::broker::mod_Comment::Src::INTERNAL { w.write_with_tag(120, |w| w.write_enum(*&self.source as i32))?; }
        Ok(())
    }
}

pub mod mod_Comment {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Src {
    INTERNAL = 0,
    EXTERNAL = 1,
}

impl Default for Src {
    fn default() -> Self {
        Src::INTERNAL
    }
}

impl From<i32> for Src {
    fn from(i: i32) -> Self {
        match i {
            0 => Src::INTERNAL,
            1 => Src::EXTERNAL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Src {
    fn from(s: &'a str) -> Self {
        match s {
            "INTERNAL" => Src::INTERNAL,
            "EXTERNAL" => Src::EXTERNAL,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    NO_TYPE = 0,
    HOST = 1,
    SERVICE = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::NO_TYPE
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::NO_TYPE,
            1 => Type::HOST,
            2 => Type::SERVICE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_TYPE" => Type::NO_TYPE,
            "HOST" => Type::HOST,
            "SERVICE" => Type::SERVICE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntryType {
    NO_ENTRY_TYPE = 0,
    USER = 1,
    DOWNTIME = 2,
    FLAPPING = 3,
    ACKNOWLEDGMENT = 4,
}

impl Default for EntryType {
    fn default() -> Self {
        EntryType::NO_ENTRY_TYPE
    }
}

impl From<i32> for EntryType {
    fn from(i: i32) -> Self {
        match i {
            0 => EntryType::NO_ENTRY_TYPE,
            1 => EntryType::USER,
            2 => EntryType::DOWNTIME,
            3 => EntryType::FLAPPING,
            4 => EntryType::ACKNOWLEDGMENT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EntryType {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_ENTRY_TYPE" => EntryType::NO_ENTRY_TYPE,
            "USER" => EntryType::USER,
            "DOWNTIME" => EntryType::DOWNTIME,
            "FLAPPING" => EntryType::FLAPPING,
            "ACKNOWLEDGMENT" => EntryType::ACKNOWLEDGMENT,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Downtime<'a> {
    pub id: u64,
    pub instance_id: u64,
    pub host_id: u64,
    pub service_id: u64,
    pub author: Cow<'a, str>,
    pub comment_data: Cow<'a, str>,
    pub type_pb: com::centreon::broker::mod_Downtime::DowntimeType,
    pub duration: u32,
    pub triggered_by: u64,
    pub entry_time: i64,
    pub actual_start_time: u64,
    pub actual_end_time: u64,
    pub start_time: u64,
    pub deletion_time: u64,
    pub end_time: u64,
    pub started: bool,
    pub cancelled: bool,
    pub fixed: bool,
}

impl<'a> MessageRead<'a> for Downtime<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(16) => msg.instance_id = r.read_uint64(bytes)?,
                Ok(24) => msg.host_id = r.read_uint64(bytes)?,
                Ok(32) => msg.service_id = r.read_uint64(bytes)?,
                Ok(42) => msg.author = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.comment_data = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.type_pb = r.read_enum(bytes)?,
                Ok(64) => msg.duration = r.read_uint32(bytes)?,
                Ok(72) => msg.triggered_by = r.read_uint64(bytes)?,
                Ok(80) => msg.entry_time = r.read_int64(bytes)?,
                Ok(88) => msg.actual_start_time = r.read_uint64(bytes)?,
                Ok(96) => msg.actual_end_time = r.read_uint64(bytes)?,
                Ok(104) => msg.start_time = r.read_uint64(bytes)?,
                Ok(112) => msg.deletion_time = r.read_uint64(bytes)?,
                Ok(120) => msg.end_time = r.read_uint64(bytes)?,
                Ok(128) => msg.started = r.read_bool(bytes)?,
                Ok(136) => msg.cancelled = r.read_bool(bytes)?,
                Ok(144) => msg.fixed = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Downtime<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.instance_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.instance_id) as u64) }
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.author == "" { 0 } else { 1 + sizeof_len((&self.author).len()) }
        + if self.comment_data == "" { 0 } else { 1 + sizeof_len((&self.comment_data).len()) }
        + if self.type_pb == com::centreon::broker::mod_Downtime::DowntimeType::NOT_USED { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.duration == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.duration) as u64) }
        + if self.triggered_by == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.triggered_by) as u64) }
        + if self.entry_time == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.entry_time) as u64) }
        + if self.actual_start_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.actual_start_time) as u64) }
        + if self.actual_end_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.actual_end_time) as u64) }
        + if self.start_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.start_time) as u64) }
        + if self.deletion_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.deletion_time) as u64) }
        + if self.end_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.end_time) as u64) }
        + if self.started == false { 0 } else { 2 + sizeof_varint(*(&self.started) as u64) }
        + if self.cancelled == false { 0 } else { 2 + sizeof_varint(*(&self.cancelled) as u64) }
        + if self.fixed == false { 0 } else { 2 + sizeof_varint(*(&self.fixed) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if self.instance_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.instance_id))?; }
        if self.host_id != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.service_id))?; }
        if self.author != "" { w.write_with_tag(42, |w| w.write_string(&**&self.author))?; }
        if self.comment_data != "" { w.write_with_tag(50, |w| w.write_string(&**&self.comment_data))?; }
        if self.type_pb != com::centreon::broker::mod_Downtime::DowntimeType::NOT_USED { w.write_with_tag(56, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.duration != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.duration))?; }
        if self.triggered_by != 0u64 { w.write_with_tag(72, |w| w.write_uint64(*&self.triggered_by))?; }
        if self.entry_time != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.entry_time))?; }
        if self.actual_start_time != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.actual_start_time))?; }
        if self.actual_end_time != 0u64 { w.write_with_tag(96, |w| w.write_uint64(*&self.actual_end_time))?; }
        if self.start_time != 0u64 { w.write_with_tag(104, |w| w.write_uint64(*&self.start_time))?; }
        if self.deletion_time != 0u64 { w.write_with_tag(112, |w| w.write_uint64(*&self.deletion_time))?; }
        if self.end_time != 0u64 { w.write_with_tag(120, |w| w.write_uint64(*&self.end_time))?; }
        if self.started != false { w.write_with_tag(128, |w| w.write_bool(*&self.started))?; }
        if self.cancelled != false { w.write_with_tag(136, |w| w.write_bool(*&self.cancelled))?; }
        if self.fixed != false { w.write_with_tag(144, |w| w.write_bool(*&self.fixed))?; }
        Ok(())
    }
}

pub mod mod_Downtime {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DowntimeType {
    NOT_USED = 0,
    SERVICE = 1,
    HOST = 2,
    ANY = 3,
}

impl Default for DowntimeType {
    fn default() -> Self {
        DowntimeType::NOT_USED
    }
}

impl From<i32> for DowntimeType {
    fn from(i: i32) -> Self {
        match i {
            0 => DowntimeType::NOT_USED,
            1 => DowntimeType::SERVICE,
            2 => DowntimeType::HOST,
            3 => DowntimeType::ANY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DowntimeType {
    fn from(s: &'a str) -> Self {
        match s {
            "NOT_USED" => DowntimeType::NOT_USED,
            "SERVICE" => DowntimeType::SERVICE,
            "HOST" => DowntimeType::HOST,
            "ANY" => DowntimeType::ANY,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CustomVariable<'a> {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub host_id: u64,
    pub service_id: u64,
    pub modified: bool,
    pub name: Cow<'a, str>,
    pub update_time: u64,
    pub value: Cow<'a, str>,
    pub default_value: Cow<'a, str>,
    pub enabled: bool,
    pub password: bool,
    pub type_pb: com::centreon::broker::mod_CustomVariable::VarType,
}

impl<'a> MessageRead<'a> for CustomVariable<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(16) => msg.host_id = r.read_uint64(bytes)?,
                Ok(24) => msg.service_id = r.read_uint64(bytes)?,
                Ok(32) => msg.modified = r.read_bool(bytes)?,
                Ok(42) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.update_time = r.read_uint64(bytes)?,
                Ok(58) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.default_value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.enabled = r.read_bool(bytes)?,
                Ok(80) => msg.password = r.read_bool(bytes)?,
                Ok(88) => msg.type_pb = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CustomVariable<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.modified == false { 0 } else { 1 + sizeof_varint(*(&self.modified) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.update_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.update_time) as u64) }
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
        + if self.default_value == "" { 0 } else { 1 + sizeof_len((&self.default_value).len()) }
        + if self.enabled == false { 0 } else { 1 + sizeof_varint(*(&self.enabled) as u64) }
        + if self.password == false { 0 } else { 1 + sizeof_varint(*(&self.password) as u64) }
        + if self.type_pb == com::centreon::broker::mod_CustomVariable::VarType::HOST { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.host_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.service_id))?; }
        if self.modified != false { w.write_with_tag(32, |w| w.write_bool(*&self.modified))?; }
        if self.name != "" { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.update_time != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.update_time))?; }
        if self.value != "" { w.write_with_tag(58, |w| w.write_string(&**&self.value))?; }
        if self.default_value != "" { w.write_with_tag(66, |w| w.write_string(&**&self.default_value))?; }
        if self.enabled != false { w.write_with_tag(72, |w| w.write_bool(*&self.enabled))?; }
        if self.password != false { w.write_with_tag(80, |w| w.write_bool(*&self.password))?; }
        if self.type_pb != com::centreon::broker::mod_CustomVariable::VarType::HOST { w.write_with_tag(88, |w| w.write_enum(*&self.type_pb as i32))?; }
        Ok(())
    }
}

pub mod mod_CustomVariable {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    HOST = 0,
    SERVICE = 1,
}

impl Default for VarType {
    fn default() -> Self {
        VarType::HOST
    }
}

impl From<i32> for VarType {
    fn from(i: i32) -> Self {
        match i {
            0 => VarType::HOST,
            1 => VarType::SERVICE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for VarType {
    fn from(s: &'a str) -> Self {
        match s {
            "HOST" => VarType::HOST,
            "SERVICE" => VarType::SERVICE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Check<'a> {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub active_checks_enabled: bool,
    pub check_type: com::centreon::broker::CheckType,
    pub command_line: Cow<'a, str>,
    pub host_id: u64,
    pub next_check: u64,
    pub service_id: u64,
}

impl<'a> MessageRead<'a> for Check<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(16) => msg.active_checks_enabled = r.read_bool(bytes)?,
                Ok(24) => msg.check_type = r.read_enum(bytes)?,
                Ok(34) => msg.command_line = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.host_id = r.read_uint64(bytes)?,
                Ok(48) => msg.next_check = r.read_uint64(bytes)?,
                Ok(56) => msg.service_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Check<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.active_checks_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.active_checks_enabled) as u64) }
        + if self.check_type == com::centreon::broker::CheckType::CheckActive { 0 } else { 1 + sizeof_varint(*(&self.check_type) as u64) }
        + if self.command_line == "" { 0 } else { 1 + sizeof_len((&self.command_line).len()) }
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.next_check == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.next_check) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.active_checks_enabled != false { w.write_with_tag(16, |w| w.write_bool(*&self.active_checks_enabled))?; }
        if self.check_type != com::centreon::broker::CheckType::CheckActive { w.write_with_tag(24, |w| w.write_enum(*&self.check_type as i32))?; }
        if self.command_line != "" { w.write_with_tag(34, |w| w.write_string(&**&self.command_line))?; }
        if self.host_id != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.host_id))?; }
        if self.next_check != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.next_check))?; }
        if self.service_id != 0u64 { w.write_with_tag(56, |w| w.write_uint64(*&self.service_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogEntry<'a> {
    pub ctime: u64,
    pub instance_name: Cow<'a, str>,
    pub output: Cow<'a, str>,
    pub host_id: u64,
    pub service_id: u64,
    pub host_name: Cow<'a, str>,
    pub service_description: Cow<'a, str>,
    pub notification_contact: Cow<'a, str>,
    pub notification_cmd: Cow<'a, str>,
    pub type_pb: com::centreon::broker::mod_LogEntry::LogType,
    pub msg_type: com::centreon::broker::mod_LogEntry::MsgType,
    pub status: i32,
    pub retry: i32,
}

impl<'a> MessageRead<'a> for LogEntry<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ctime = r.read_uint64(bytes)?,
                Ok(18) => msg.instance_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.output = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.host_id = r.read_uint64(bytes)?,
                Ok(40) => msg.service_id = r.read_uint64(bytes)?,
                Ok(50) => msg.host_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.service_description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.notification_contact = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.notification_cmd = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.type_pb = r.read_enum(bytes)?,
                Ok(88) => msg.msg_type = r.read_enum(bytes)?,
                Ok(96) => msg.status = r.read_int32(bytes)?,
                Ok(104) => msg.retry = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogEntry<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ctime == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ctime) as u64) }
        + if self.instance_name == "" { 0 } else { 1 + sizeof_len((&self.instance_name).len()) }
        + if self.output == "" { 0 } else { 1 + sizeof_len((&self.output).len()) }
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.host_name == "" { 0 } else { 1 + sizeof_len((&self.host_name).len()) }
        + if self.service_description == "" { 0 } else { 1 + sizeof_len((&self.service_description).len()) }
        + if self.notification_contact == "" { 0 } else { 1 + sizeof_len((&self.notification_contact).len()) }
        + if self.notification_cmd == "" { 0 } else { 1 + sizeof_len((&self.notification_cmd).len()) }
        + if self.type_pb == com::centreon::broker::mod_LogEntry::LogType::SOFT { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.msg_type == com::centreon::broker::mod_LogEntry::MsgType::SERVICE_ALERT { 0 } else { 1 + sizeof_varint(*(&self.msg_type) as u64) }
        + if self.status == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.retry == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retry) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ctime != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.ctime))?; }
        if self.instance_name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.instance_name))?; }
        if self.output != "" { w.write_with_tag(26, |w| w.write_string(&**&self.output))?; }
        if self.host_id != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.service_id))?; }
        if self.host_name != "" { w.write_with_tag(50, |w| w.write_string(&**&self.host_name))?; }
        if self.service_description != "" { w.write_with_tag(58, |w| w.write_string(&**&self.service_description))?; }
        if self.notification_contact != "" { w.write_with_tag(66, |w| w.write_string(&**&self.notification_contact))?; }
        if self.notification_cmd != "" { w.write_with_tag(74, |w| w.write_string(&**&self.notification_cmd))?; }
        if self.type_pb != com::centreon::broker::mod_LogEntry::LogType::SOFT { w.write_with_tag(80, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.msg_type != com::centreon::broker::mod_LogEntry::MsgType::SERVICE_ALERT { w.write_with_tag(88, |w| w.write_enum(*&self.msg_type as i32))?; }
        if self.status != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.status))?; }
        if self.retry != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.retry))?; }
        Ok(())
    }
}

pub mod mod_LogEntry {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogType {
    SOFT = 0,
    HARD = 1,
}

impl Default for LogType {
    fn default() -> Self {
        LogType::SOFT
    }
}

impl From<i32> for LogType {
    fn from(i: i32) -> Self {
        match i {
            0 => LogType::SOFT,
            1 => LogType::HARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for LogType {
    fn from(s: &'a str) -> Self {
        match s {
            "SOFT" => LogType::SOFT,
            "HARD" => LogType::HARD,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MsgType {
    SERVICE_ALERT = 0,
    HOST_ALERT = 1,
    SERVICE_NOTIFICATION = 2,
    HOST_NOTIFICATION = 3,
    WARNING = 4,
    OTHER = 5,
    SERVICE_INITIAL_STATE = 8,
    HOST_INITIAL_STATE = 9,
    SERVICE_ACKNOWLEDGE_PROBLEM = 10,
    HOST_ACKNOWLEDGE_PROBLEM = 11,
    SERVICE_EVENT_HANDLER = 12,
    HOST_EVENT_HANDLER = 13,
    GLOBAL_SERVICE_EVENT_HANDLER = 14,
    GLOBAL_HOST_EVENT_HANDLER = 15,
}

impl Default for MsgType {
    fn default() -> Self {
        MsgType::SERVICE_ALERT
    }
}

impl From<i32> for MsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => MsgType::SERVICE_ALERT,
            1 => MsgType::HOST_ALERT,
            2 => MsgType::SERVICE_NOTIFICATION,
            3 => MsgType::HOST_NOTIFICATION,
            4 => MsgType::WARNING,
            5 => MsgType::OTHER,
            8 => MsgType::SERVICE_INITIAL_STATE,
            9 => MsgType::HOST_INITIAL_STATE,
            10 => MsgType::SERVICE_ACKNOWLEDGE_PROBLEM,
            11 => MsgType::HOST_ACKNOWLEDGE_PROBLEM,
            12 => MsgType::SERVICE_EVENT_HANDLER,
            13 => MsgType::HOST_EVENT_HANDLER,
            14 => MsgType::GLOBAL_SERVICE_EVENT_HANDLER,
            15 => MsgType::GLOBAL_HOST_EVENT_HANDLER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "SERVICE_ALERT" => MsgType::SERVICE_ALERT,
            "HOST_ALERT" => MsgType::HOST_ALERT,
            "SERVICE_NOTIFICATION" => MsgType::SERVICE_NOTIFICATION,
            "HOST_NOTIFICATION" => MsgType::HOST_NOTIFICATION,
            "WARNING" => MsgType::WARNING,
            "OTHER" => MsgType::OTHER,
            "SERVICE_INITIAL_STATE" => MsgType::SERVICE_INITIAL_STATE,
            "HOST_INITIAL_STATE" => MsgType::HOST_INITIAL_STATE,
            "SERVICE_ACKNOWLEDGE_PROBLEM" => MsgType::SERVICE_ACKNOWLEDGE_PROBLEM,
            "HOST_ACKNOWLEDGE_PROBLEM" => MsgType::HOST_ACKNOWLEDGE_PROBLEM,
            "SERVICE_EVENT_HANDLER" => MsgType::SERVICE_EVENT_HANDLER,
            "HOST_EVENT_HANDLER" => MsgType::HOST_EVENT_HANDLER,
            "GLOBAL_SERVICE_EVENT_HANDLER" => MsgType::GLOBAL_SERVICE_EVENT_HANDLER,
            "GLOBAL_HOST_EVENT_HANDLER" => MsgType::GLOBAL_HOST_EVENT_HANDLER,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InstanceStatus<'a> {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub event_handlers: bool,
    pub flap_detection: bool,
    pub notifications: bool,
    pub active_host_checks: bool,
    pub active_service_checks: bool,
    pub check_hosts_freshness: bool,
    pub check_services_freshness: bool,
    pub global_host_event_handler: Cow<'a, str>,
    pub global_service_event_handler: Cow<'a, str>,
    pub last_alive: u64,
    pub last_command_check: i64,
    pub obsess_over_hosts: bool,
    pub obsess_over_services: bool,
    pub passive_host_checks: bool,
    pub passive_service_checks: bool,
    pub instance_id: u64,
}

impl<'a> MessageRead<'a> for InstanceStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(16) => msg.event_handlers = r.read_bool(bytes)?,
                Ok(24) => msg.flap_detection = r.read_bool(bytes)?,
                Ok(32) => msg.notifications = r.read_bool(bytes)?,
                Ok(40) => msg.active_host_checks = r.read_bool(bytes)?,
                Ok(48) => msg.active_service_checks = r.read_bool(bytes)?,
                Ok(56) => msg.check_hosts_freshness = r.read_bool(bytes)?,
                Ok(64) => msg.check_services_freshness = r.read_bool(bytes)?,
                Ok(74) => msg.global_host_event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.global_service_event_handler = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.last_alive = r.read_uint64(bytes)?,
                Ok(96) => msg.last_command_check = r.read_int64(bytes)?,
                Ok(104) => msg.obsess_over_hosts = r.read_bool(bytes)?,
                Ok(112) => msg.obsess_over_services = r.read_bool(bytes)?,
                Ok(120) => msg.passive_host_checks = r.read_bool(bytes)?,
                Ok(128) => msg.passive_service_checks = r.read_bool(bytes)?,
                Ok(136) => msg.instance_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for InstanceStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.event_handlers == false { 0 } else { 1 + sizeof_varint(*(&self.event_handlers) as u64) }
        + if self.flap_detection == false { 0 } else { 1 + sizeof_varint(*(&self.flap_detection) as u64) }
        + if self.notifications == false { 0 } else { 1 + sizeof_varint(*(&self.notifications) as u64) }
        + if self.active_host_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_host_checks) as u64) }
        + if self.active_service_checks == false { 0 } else { 1 + sizeof_varint(*(&self.active_service_checks) as u64) }
        + if self.check_hosts_freshness == false { 0 } else { 1 + sizeof_varint(*(&self.check_hosts_freshness) as u64) }
        + if self.check_services_freshness == false { 0 } else { 1 + sizeof_varint(*(&self.check_services_freshness) as u64) }
        + if self.global_host_event_handler == "" { 0 } else { 1 + sizeof_len((&self.global_host_event_handler).len()) }
        + if self.global_service_event_handler == "" { 0 } else { 1 + sizeof_len((&self.global_service_event_handler).len()) }
        + if self.last_alive == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.last_alive) as u64) }
        + if self.last_command_check == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_command_check) as u64) }
        + if self.obsess_over_hosts == false { 0 } else { 1 + sizeof_varint(*(&self.obsess_over_hosts) as u64) }
        + if self.obsess_over_services == false { 0 } else { 1 + sizeof_varint(*(&self.obsess_over_services) as u64) }
        + if self.passive_host_checks == false { 0 } else { 1 + sizeof_varint(*(&self.passive_host_checks) as u64) }
        + if self.passive_service_checks == false { 0 } else { 2 + sizeof_varint(*(&self.passive_service_checks) as u64) }
        + if self.instance_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.instance_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.event_handlers != false { w.write_with_tag(16, |w| w.write_bool(*&self.event_handlers))?; }
        if self.flap_detection != false { w.write_with_tag(24, |w| w.write_bool(*&self.flap_detection))?; }
        if self.notifications != false { w.write_with_tag(32, |w| w.write_bool(*&self.notifications))?; }
        if self.active_host_checks != false { w.write_with_tag(40, |w| w.write_bool(*&self.active_host_checks))?; }
        if self.active_service_checks != false { w.write_with_tag(48, |w| w.write_bool(*&self.active_service_checks))?; }
        if self.check_hosts_freshness != false { w.write_with_tag(56, |w| w.write_bool(*&self.check_hosts_freshness))?; }
        if self.check_services_freshness != false { w.write_with_tag(64, |w| w.write_bool(*&self.check_services_freshness))?; }
        if self.global_host_event_handler != "" { w.write_with_tag(74, |w| w.write_string(&**&self.global_host_event_handler))?; }
        if self.global_service_event_handler != "" { w.write_with_tag(82, |w| w.write_string(&**&self.global_service_event_handler))?; }
        if self.last_alive != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.last_alive))?; }
        if self.last_command_check != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.last_command_check))?; }
        if self.obsess_over_hosts != false { w.write_with_tag(104, |w| w.write_bool(*&self.obsess_over_hosts))?; }
        if self.obsess_over_services != false { w.write_with_tag(112, |w| w.write_bool(*&self.obsess_over_services))?; }
        if self.passive_host_checks != false { w.write_with_tag(120, |w| w.write_bool(*&self.passive_host_checks))?; }
        if self.passive_service_checks != false { w.write_with_tag(128, |w| w.write_bool(*&self.passive_service_checks))?; }
        if self.instance_id != 0u64 { w.write_with_tag(136, |w| w.write_uint64(*&self.instance_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Instance<'a> {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub engine: Cow<'a, str>,
    pub running: bool,
    pub name: Cow<'a, str>,
    pub pid: i64,
    pub instance_id: u64,
    pub end_time: i64,
    pub start_time: i64,
    pub version: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Instance<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(18) => msg.engine = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.running = r.read_bool(bytes)?,
                Ok(34) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.pid = r.read_int64(bytes)?,
                Ok(48) => msg.instance_id = r.read_uint64(bytes)?,
                Ok(56) => msg.end_time = r.read_int64(bytes)?,
                Ok(64) => msg.start_time = r.read_int64(bytes)?,
                Ok(74) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Instance<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.engine == "" { 0 } else { 1 + sizeof_len((&self.engine).len()) }
        + if self.running == false { 0 } else { 1 + sizeof_varint(*(&self.running) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.pid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.pid) as u64) }
        + if self.instance_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.instance_id) as u64) }
        + if self.end_time == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.end_time) as u64) }
        + if self.start_time == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.start_time) as u64) }
        + if self.version == "" { 0 } else { 1 + sizeof_len((&self.version).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.engine != "" { w.write_with_tag(18, |w| w.write_string(&**&self.engine))?; }
        if self.running != false { w.write_with_tag(24, |w| w.write_bool(*&self.running))?; }
        if self.name != "" { w.write_with_tag(34, |w| w.write_string(&**&self.name))?; }
        if self.pid != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.pid))?; }
        if self.instance_id != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.instance_id))?; }
        if self.end_time != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.end_time))?; }
        if self.start_time != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.start_time))?; }
        if self.version != "" { w.write_with_tag(74, |w| w.write_string(&**&self.version))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponsiveInstance {
    pub header: Option<com::centreon::broker::BBDOHeader>,
    pub poller_id: u64,
    pub responsive: bool,
}

impl<'a> MessageRead<'a> for ResponsiveInstance {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<com::centreon::broker::BBDOHeader>(bytes)?),
                Ok(16) => msg.poller_id = r.read_uint64(bytes)?,
                Ok(24) => msg.responsive = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponsiveInstance {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.poller_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.poller_id) as u64) }
        + if self.responsive == false { 0 } else { 1 + sizeof_varint(*(&self.responsive) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.poller_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.poller_id))?; }
        if self.responsive != false { w.write_with_tag(24, |w| w.write_bool(*&self.responsive))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Acknowledgement<'a> {
    pub host_id: u64,
    pub service_id: u64,
    pub instance_id: u64,
    pub type_pb: com::centreon::broker::mod_Acknowledgement::AcknowledgementType,
    pub author: Cow<'a, str>,
    pub comment_data: Cow<'a, str>,
    pub sticky: bool,
    pub notify_contacts: bool,
    pub entry_time: u64,
    pub deletion_time: u64,
    pub persistent_comment: bool,
    pub state: i32,
}

impl<'a> MessageRead<'a> for Acknowledgement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.host_id = r.read_uint64(bytes)?,
                Ok(16) => msg.service_id = r.read_uint64(bytes)?,
                Ok(24) => msg.instance_id = r.read_uint64(bytes)?,
                Ok(32) => msg.type_pb = r.read_enum(bytes)?,
                Ok(42) => msg.author = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.comment_data = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.sticky = r.read_bool(bytes)?,
                Ok(64) => msg.notify_contacts = r.read_bool(bytes)?,
                Ok(72) => msg.entry_time = r.read_uint64(bytes)?,
                Ok(80) => msg.deletion_time = r.read_uint64(bytes)?,
                Ok(88) => msg.persistent_comment = r.read_bool(bytes)?,
                Ok(96) => msg.state = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Acknowledgement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.host_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.host_id) as u64) }
        + if self.service_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.service_id) as u64) }
        + if self.instance_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.instance_id) as u64) }
        + if self.type_pb == com::centreon::broker::mod_Acknowledgement::AcknowledgementType::NONE { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.author == "" { 0 } else { 1 + sizeof_len((&self.author).len()) }
        + if self.comment_data == "" { 0 } else { 1 + sizeof_len((&self.comment_data).len()) }
        + if self.sticky == false { 0 } else { 1 + sizeof_varint(*(&self.sticky) as u64) }
        + if self.notify_contacts == false { 0 } else { 1 + sizeof_varint(*(&self.notify_contacts) as u64) }
        + if self.entry_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.entry_time) as u64) }
        + if self.deletion_time == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.deletion_time) as u64) }
        + if self.persistent_comment == false { 0 } else { 1 + sizeof_varint(*(&self.persistent_comment) as u64) }
        + if self.state == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.host_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.host_id))?; }
        if self.service_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.service_id))?; }
        if self.instance_id != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.instance_id))?; }
        if self.type_pb != com::centreon::broker::mod_Acknowledgement::AcknowledgementType::NONE { w.write_with_tag(32, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.author != "" { w.write_with_tag(42, |w| w.write_string(&**&self.author))?; }
        if self.comment_data != "" { w.write_with_tag(50, |w| w.write_string(&**&self.comment_data))?; }
        if self.sticky != false { w.write_with_tag(56, |w| w.write_bool(*&self.sticky))?; }
        if self.notify_contacts != false { w.write_with_tag(64, |w| w.write_bool(*&self.notify_contacts))?; }
        if self.entry_time != 0u64 { w.write_with_tag(72, |w| w.write_uint64(*&self.entry_time))?; }
        if self.deletion_time != 0u64 { w.write_with_tag(80, |w| w.write_uint64(*&self.deletion_time))?; }
        if self.persistent_comment != false { w.write_with_tag(88, |w| w.write_bool(*&self.persistent_comment))?; }
        if self.state != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.state))?; }
        Ok(())
    }
}

pub mod mod_Acknowledgement {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AcknowledgementType {
    NONE = 0,
    NORMAL = 1,
    STICKY = 2,
}

impl Default for AcknowledgementType {
    fn default() -> Self {
        AcknowledgementType::NONE
    }
}

impl From<i32> for AcknowledgementType {
    fn from(i: i32) -> Self {
        match i {
            0 => AcknowledgementType::NONE,
            1 => AcknowledgementType::NORMAL,
            2 => AcknowledgementType::STICKY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for AcknowledgementType {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => AcknowledgementType::NONE,
            "NORMAL" => AcknowledgementType::NORMAL,
            "STICKY" => AcknowledgementType::STICKY,
            _ => Self::default(),
        }
    }
}

}

