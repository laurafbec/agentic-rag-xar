#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__AddWp_Request() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__AddWp_Request__init(msg: *mut AddWp_Request) -> bool;
    fn waypoint_navigation_msgs__srv__AddWp_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddWp_Request>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__AddWp_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddWp_Request>);
    fn waypoint_navigation_msgs__srv__AddWp_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddWp_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddWp_Request>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__AddWp_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddWp_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp: super::super::msg::rmw::Wp,

}



impl Default for AddWp_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__AddWp_Request__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__AddWp_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddWp_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddWp_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddWp_Request where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/AddWp_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__AddWp_Request() }
  }
}


#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__AddWp_Response() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__AddWp_Response__init(msg: *mut AddWp_Response) -> bool;
    fn waypoint_navigation_msgs__srv__AddWp_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddWp_Response>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__AddWp_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddWp_Response>);
    fn waypoint_navigation_msgs__srv__AddWp_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddWp_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddWp_Response>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__AddWp_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddWp_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub overwrite: bool,

}



impl Default for AddWp_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__AddWp_Response__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__AddWp_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddWp_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__AddWp_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddWp_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddWp_Response where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/AddWp_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__AddWp_Response() }
  }
}


#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWp_Request() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__GetWp_Request__init(msg: *mut GetWp_Request) -> bool;
    fn waypoint_navigation_msgs__srv__GetWp_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWp_Request>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__GetWp_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWp_Request>);
    fn waypoint_navigation_msgs__srv__GetWp_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWp_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWp_Request>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWp_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWp_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp_id: rosidl_runtime_rs::String,

}



impl Default for GetWp_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__GetWp_Request__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__GetWp_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWp_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWp_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWp_Request where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/GetWp_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWp_Request() }
  }
}


#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWp_Response() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__GetWp_Response__init(msg: *mut GetWp_Response) -> bool;
    fn waypoint_navigation_msgs__srv__GetWp_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWp_Response>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__GetWp_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWp_Response>);
    fn waypoint_navigation_msgs__srv__GetWp_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWp_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWp_Response>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWp_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWp_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp: super::super::msg::rmw::Wp,

}



impl Default for GetWp_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__GetWp_Response__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__GetWp_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWp_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWp_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWp_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWp_Response where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/GetWp_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWp_Response() }
  }
}


#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWps_Request() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__GetWps_Request__init(msg: *mut GetWps_Request) -> bool;
    fn waypoint_navigation_msgs__srv__GetWps_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWps_Request>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__GetWps_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWps_Request>);
    fn waypoint_navigation_msgs__srv__GetWps_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWps_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWps_Request>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWps_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetWps_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__GetWps_Request__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__GetWps_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWps_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWps_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWps_Request where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/GetWps_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWps_Request() }
  }
}


#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWps_Response() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation_msgs__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation_msgs__srv__GetWps_Response__init(msg: *mut GetWps_Response) -> bool;
    fn waypoint_navigation_msgs__srv__GetWps_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWps_Response>, size: usize) -> bool;
    fn waypoint_navigation_msgs__srv__GetWps_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWps_Response>);
    fn waypoint_navigation_msgs__srv__GetWps_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWps_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWps_Response>) -> bool;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWps_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wps: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Wp>,

}



impl Default for GetWps_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation_msgs__srv__GetWps_Response__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation_msgs__srv__GetWps_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWps_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation_msgs__srv__GetWps_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWps_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWps_Response where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation_msgs/srv/GetWps_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation_msgs__srv__GetWps_Response() }
  }
}






#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__AddWp() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation_msgs__srv__AddWp
#[allow(missing_docs, non_camel_case_types)]
pub struct AddWp;

impl rosidl_runtime_rs::Service for AddWp {
    type Request = AddWp_Request;
    type Response = AddWp_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__AddWp() }
    }
}




#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__GetWp() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWp
#[allow(missing_docs, non_camel_case_types)]
pub struct GetWp;

impl rosidl_runtime_rs::Service for GetWp {
    type Request = GetWp_Request;
    type Response = GetWp_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__GetWp() }
    }
}




#[link(name = "waypoint_navigation_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__GetWps() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation_msgs__srv__GetWps
#[allow(missing_docs, non_camel_case_types)]
pub struct GetWps;

impl rosidl_runtime_rs::Service for GetWps {
    type Request = GetWps_Request;
    type Response = GetWps_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation_msgs__srv__GetWps() }
    }
}


