// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from waypoint_navigation_msgs:srv/GetWps.idl
// generated code does not contain a copyright notice

#ifndef WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__BUILDER_HPP_
#define WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "waypoint_navigation_msgs/srv/detail/get_wps__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace waypoint_navigation_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::waypoint_navigation_msgs::srv::GetWps_Request>()
{
  return ::waypoint_navigation_msgs::srv::GetWps_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace waypoint_navigation_msgs


namespace waypoint_navigation_msgs
{

namespace srv
{

namespace builder
{

class Init_GetWps_Response_wps
{
public:
  Init_GetWps_Response_wps()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::waypoint_navigation_msgs::srv::GetWps_Response wps(::waypoint_navigation_msgs::srv::GetWps_Response::_wps_type arg)
  {
    msg_.wps = std::move(arg);
    return std::move(msg_);
  }

private:
  ::waypoint_navigation_msgs::srv::GetWps_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::waypoint_navigation_msgs::srv::GetWps_Response>()
{
  return waypoint_navigation_msgs::srv::builder::Init_GetWps_Response_wps();
}

}  // namespace waypoint_navigation_msgs

#endif  // WAYPOINT_NAVIGATION_MSGS__SRV__DETAIL__GET_WPS__BUILDER_HPP_
