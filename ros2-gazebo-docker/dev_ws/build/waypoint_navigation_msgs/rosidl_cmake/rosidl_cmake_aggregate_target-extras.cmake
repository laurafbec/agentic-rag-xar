# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target waypoint_navigation_msgs::waypoint_navigation_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${waypoint_navigation_msgs_TARGETS}.
if(waypoint_navigation_msgs_TARGETS AND NOT TARGET waypoint_navigation_msgs::waypoint_navigation_msgs)
  add_library(waypoint_navigation_msgs::waypoint_navigation_msgs INTERFACE IMPORTED)
  set_target_properties(waypoint_navigation_msgs::waypoint_navigation_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${waypoint_navigation_msgs_TARGETS}")
endif()
