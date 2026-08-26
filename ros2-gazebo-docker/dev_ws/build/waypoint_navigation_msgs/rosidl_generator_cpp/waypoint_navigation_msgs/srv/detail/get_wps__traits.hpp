// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from waypoint_navigation_msgs:srv/GetWps.idl
// generated code does not contain a copyright notice

#ifndef WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__TRAITS_HPP_
#define WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "waypoint_navigation_msgs/srv/detail/get_wps__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace waypoint_navigation_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetWps_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetWps_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetWps_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace waypoint_navigation_msgs

namespace rosidl_generator_traits
{

[[deprecated("use waypoint_navigation_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const waypoint_navigation_msgs::srv::GetWps_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  waypoint_navigation_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use waypoint_navigation_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const waypoint_navigation_msgs::srv::GetWps_Request & msg)
{
  return waypoint_navigation_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::GetWps_Request>()
{
  return "waypoint_navigation_msgs::srv::GetWps_Request";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::GetWps_Request>()
{
  return "waypoint_navigation_msgs/srv/GetWps_Request";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::GetWps_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::GetWps_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<waypoint_navigation_msgs::srv::GetWps_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'wps'
#include "waypoint_navigation_msgs/msg/detail/wp__traits.hpp"

namespace waypoint_navigation_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetWps_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: wps
  {
    if (msg.wps.size() == 0) {
      out << "wps: []";
    } else {
      out << "wps: [";
      size_t pending_items = msg.wps.size();
      for (auto item : msg.wps) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetWps_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: wps
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.wps.size() == 0) {
      out << "wps: []\n";
    } else {
      out << "wps:\n";
      for (auto item : msg.wps) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetWps_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace waypoint_navigation_msgs

namespace rosidl_generator_traits
{

[[deprecated("use waypoint_navigation_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const waypoint_navigation_msgs::srv::GetWps_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  waypoint_navigation_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use waypoint_navigation_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const waypoint_navigation_msgs::srv::GetWps_Response & msg)
{
  return waypoint_navigation_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::GetWps_Response>()
{
  return "waypoint_navigation_msgs::srv::GetWps_Response";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::GetWps_Response>()
{
  return "waypoint_navigation_msgs/srv/GetWps_Response";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::GetWps_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::GetWps_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<waypoint_navigation_msgs::srv::GetWps_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::GetWps>()
{
  return "waypoint_navigation_msgs::srv::GetWps";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::GetWps>()
{
  return "waypoint_navigation_msgs/srv/GetWps";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::GetWps>
  : std::integral_constant<
    bool,
    has_fixed_size<waypoint_navigation_msgs::srv::GetWps_Request>::value &&
    has_fixed_size<waypoint_navigation_msgs::srv::GetWps_Response>::value
  >
{
};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::GetWps>
  : std::integral_constant<
    bool,
    has_bounded_size<waypoint_navigation_msgs::srv::GetWps_Request>::value &&
    has_bounded_size<waypoint_navigation_msgs::srv::GetWps_Response>::value
  >
{
};

template<>
struct is_service<waypoint_navigation_msgs::srv::GetWps>
  : std::true_type
{
};

template<>
struct is_service_request<waypoint_navigation_msgs::srv::GetWps_Request>
  : std::true_type
{
};

template<>
struct is_service_response<waypoint_navigation_msgs::srv::GetWps_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__TRAITS_HPP_
