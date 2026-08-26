from setuptools import find_packages
from setuptools import setup

setup(
    name='waypoint_navigation_msgs',
    version='0.0.0',
    packages=find_packages(
        include=('waypoint_navigation_msgs', 'waypoint_navigation_msgs.*')),
)
