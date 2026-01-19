# enlil
rust async framework w/ observability, maintainability, documentation built in

## modules

This framework asks you to compose code into modules (not rust modules). The module defines its outputs and state explicitly. Each module is run either on demand (its dependencies change) or periodically. Helpful boilerplate is built around the module: 

* observability with open telemetry
* a data dictionary
* module descriptions
* interdependency of modules viewed as a web
* inter-module communication
* failure recreate
* module testing framework

Additional benefits of this framework allow for modules to be dynamically swapped or restarted. 

## services

Modules are contained inside a service. A service is an independent process that runs the modules using async. The service starts the modules and Common Modules can be instantiated to do the following:

* push service status updates to a message broker
* push logged events to a message broker
* subscribe to data in a message broker
* start and stop other modules


Additionally, when a modules experiences an unanticipated failure, the service writes the preceding state and inputs to file so the failure can be re-created offline.

## observability

Each module and service is integrated with open telemetry observability. Tracing is automatic for each module, and the service as a whole. Variables can become metrics when they are configured in the modules. Each module can have independent log levels for open telemetry logs vs logs that are kept for the system.

## documentation

The implementation of each module requires documenting each important variable right where you use it in code. This documentation is extractable and converts to a data dictionary. The data interdependence is also mapped and included in the dictionary.

Because each module explicitly depends on other data, the module data flow can be visualized in a web. This can be used to inspect for proper design. Included in the module map are descriptions of the modules.

Modules that push data as "service status" are marked inside their configuration allowing the service to handle updating any configured messenger services. Implementing a Common Module to subscribe to messenger data allows an async api document to be generated for the messenger service.

## testing

Having clear module interfaces allows modules to be tested independently.

When a module fails in production, its inputs and state are captured for re-creation. This sequence (failure capture) can be used immediately as a test case for the module, allowing for recreation, validation, and as a test case.

The failure capture method can also be used to generate test cases. Simply trigger the capture durning normal operation, change the input or initial state, and generate the failure condition that is expected.

Modules that do not perform hardware communication can be fully integration tested in this way. This limits detailed integration testing to only the processes that interact with hardware.

Traditional methods involving mocking, hardware in the loop, and full system testing can be used to round out coverage. However these methods are progressively more expensive and can be minimized with more low level integration testing.