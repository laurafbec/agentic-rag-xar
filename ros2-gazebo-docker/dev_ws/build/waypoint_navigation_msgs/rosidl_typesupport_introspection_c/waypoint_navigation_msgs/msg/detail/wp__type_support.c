// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from waypoint_navigation_msgs:msg/Wp.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "waypoint_navigation_msgs/msg/detail/wp__rosidl_typesupport_introspection_c.h"
#include "waypoint_navigation_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "waypoint_navigation_msgs/msg/detail/wp__functions.h"
#include "waypoint_navigation_msgs/msg/detail/wp__struct.h"


// Include directives for member types
// Member `id`
#include "rosidl_runtime_c/string_functions.h"
// Member `pose`
#include "geometry_msgs/msg/pose.h"
// Member `pose`
#include "geometry_msgs/msg/detail/pose__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  waypoint_navigation_msgs__msg__Wp__init(message_memory);
}

void waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_fini_function(void * message_memory)
{
  waypoint_navigation_msgs__msg__Wp__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_member_array[2] = {
  {
    "id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(waypoint_navigation_msgs__msg__Wp, id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "pose",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(waypoint_navigation_msgs__msg__Wp, pose),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_members = {
  "waypoint_navigation_msgs__msg",  // message namespace
  "Wp",  // message name
  2,  // number of fields
  sizeof(waypoint_navigation_msgs__msg__Wp),
  waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_member_array,  // message members
  waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_init_function,  // function to initialize message memory (memory has to be allocated)
  waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_type_support_handle = {
  0,
  &waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_waypoint_navigation_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, waypoint_navigation_msgs, msg, Wp)() {
  waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, geometry_msgs, msg, Pose)();
  if (!waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_type_support_handle.typesupport_identifier) {
    waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &waypoint_navigation_msgs__msg__Wp__rosidl_typesupport_introspection_c__Wp_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
