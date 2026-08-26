#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to waypoint_navigation_msgs__srv__AddWp_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddWp_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp: super::msg::Wp,

}



impl Default for AddWp_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddWp_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddWp_Request {
  type RmwMsg = super::srv::rmw::AddWp_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp: super::msg::Wp::into_rmw_message(std::borrow::Cow::Owned(msg.wp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp: super::msg::Wp::into_rmw_message(std::borrow::Cow::Borrowed(&msg.wp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      wp: super::msg::Wp::from_rmw_message(msg.wp),
    }
  }
}


// Corresponds to waypoint_navigation_msgs__srv__AddWp_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddWp_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub overwrite: bool,

}



impl Default for AddWp_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddWp_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddWp_Response {
  type RmwMsg = super::srv::rmw::AddWp_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        overwrite: msg.overwrite,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      overwrite: msg.overwrite,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      overwrite: msg.overwrite,
    }
  }
}


// Corresponds to waypoint_navigation_msgs__srv__GetWp_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWp_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp_id: std::string::String,

}



impl Default for GetWp_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWp_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetWp_Request {
  type RmwMsg = super::srv::rmw::GetWp_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp_id: msg.wp_id.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp_id: msg.wp_id.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      wp_id: msg.wp_id.to_string(),
    }
  }
}


// Corresponds to waypoint_navigation_msgs__srv__GetWp_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWp_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wp: super::msg::Wp,

}



impl Default for GetWp_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWp_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetWp_Response {
  type RmwMsg = super::srv::rmw::GetWp_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp: super::msg::Wp::into_rmw_message(std::borrow::Cow::Owned(msg.wp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wp: super::msg::Wp::into_rmw_message(std::borrow::Cow::Borrowed(&msg.wp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      wp: super::msg::Wp::from_rmw_message(msg.wp),
    }
  }
}


// Corresponds to waypoint_navigation_msgs__srv__GetWps_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetWps_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWps_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetWps_Request {
  type RmwMsg = super::srv::rmw::GetWps_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to waypoint_navigation_msgs__srv__GetWps_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub wps: Vec<super::msg::Wp>,

}



impl Default for GetWps_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWps_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetWps_Response {
  type RmwMsg = super::srv::rmw::GetWps_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wps: msg.wps
          .into_iter()
          .map(|elem| super::msg::Wp::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        wps: msg.wps
          .iter()
          .map(|elem| super::msg::Wp::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      wps: msg.wps
          .into_iter()
          .map(super::msg::Wp::from_rmw_message)
          .collect(),
    }
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


