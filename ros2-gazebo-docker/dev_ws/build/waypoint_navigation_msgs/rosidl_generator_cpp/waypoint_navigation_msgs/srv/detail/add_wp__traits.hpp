// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from waypoint_navigation_msgs:srv/AddWp.idl
// generated code does not contain a copyright notice

#ifndef WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__ADD_WP__TRAITS_HPP_
#define WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__ADD_WP__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "waypoint_navigation_msgs/srv/detail/add_wp__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'wp'
#include "waypoint_navigation_msgs/msg/detail/wp__traits.hpp"

namespace waypoint_navigation_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const AddWp_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: wp
  {
    out << "wp: ";
    to_flow_style_yaml(msg.wp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AddWp_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: wp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "wp:\n";
    to_block_style_yaml(msg.wp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AddWp_Request & msg, bool use_flow_style = false)
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
  const waypoint_navigation_msgs::srv::AddWp_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  waypoint_navigation_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use waypoint_navigation_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const waypoint_navigation_msgs::srv::AddWp_Request & msg)
{
  return waypoint_navigation_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::AddWp_Request>()
{
  return "waypoint_navigation_msgs::srv::AddWp_Request";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::AddWp_Request>()
{
  return "waypoint_navigation_msgs/srv/AddWp_Request";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::AddWp_Request>
  : std::integral_constant<bool, has_fixed_size<waypoint_navigation_msgs::msg::Wp>::value> {};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::AddWp_Request>
  : std::integral_constant<bool, has_bounded_size<waypoint_navigation_msgs::msg::Wp>::value> {};

template<>
struct is_message<waypoint_navigation_msgs::srv::AddWp_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace waypoint_navigation_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const AddWp_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: overwrite
  {
    out << "overwrite: ";
    rosidl_generator_traits::value_to_yaml(msg.overwrite, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AddWp_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: overwrite
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "overwrite: ";
    rosidl_generator_traits::value_to_yaml(msg.overwrite, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AddWp_Response & msg, bool use_flow_style = false)
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
  const waypoint_navigation_msgs::srv::AddWp_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  waypoint_navigation_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use waypoint_navigation_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const waypoint_navigation_msgs::srv::AddWp_Response & msg)
{
  return waypoint_navigation_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::AddWp_Response>()
{
  return "waypoint_navigation_msgs::srv::AddWp_Response";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::AddWp_Response>()
{
  return "waypoint_navigation_msgs/srv/AddWp_Response";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::AddWp_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::AddWp_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<waypoint_navigation_msgs::srv::AddWp_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<waypoint_navigation_msgs::srv::AddWp>()
{
  return "waypoint_navigation_msgs::srv::AddWp";
}

template<>
inline const char * name<waypoint_navigation_msgs::srv::AddWp>()
{
  return "waypoint_navigation_msgs/srv/AddWp";
}

template<>
struct has_fixed_size<waypoint_navigation_msgs::srv::AddWp>
  : std::integral_constant<
    bool,
    has_fixed_size<waypoint_navigation_msgs::srv::AddWp_Request>::value &&
    has_fixed_size<waypoint_navigation_msgs::srv::AddWp_Response>::value
  >
{
};

template<>
struct has_bounded_size<waypoint_navigation_msgs::srv::AddWp>
  : std::integral_constant<
    bool,
    has_bounded_size<waypoint_navigation_msgs::srv::AddWp_Request>::value &&
    has_bounded_size<waypoint_navigation_msgs::srv::AddWp_Response>::value
  >
{
};

template<>
struct is_service<waypoint_navigation_msgs::srv::AddWp>
  : std::true_type
{
};

template<>
struct is_service_request<waypoint_navigation_msgs::srv::AddWp_Request>
  : std::true_type
{
};

template<>
struct is_service_response<waypoint_navigation_msgs::srv::AddWp_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__ADD_WP__TRAITS_HPP_
