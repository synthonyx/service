# Software Design with Rust

Copyright (c) 2024, Synthonyx Technologies Ltd.

## Licensing

Double licensed under the Apache-2.0 and MIT license.

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
```

```
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Description

This service starter kit is a great starting point for building services that can handle a large number of users and are designed to grow with your needs. It uses of a domain driven approach, which allows for flexible and modular development. By separating the business logic from other concerns, the project follows a proven approach.

The kit is written in Rust, a powerful and modern programming language, and aims to reduce repetitive tasks and provide a solid foundation for launching your project into production. It uses minimal dependencies, adopts best practices, and is designed to be intuitive for Rust developers.

The accompanying course goes beyond the basics and teaches you how to build production-level software in Rust using a domain driven approach. You will work closely with an instructor who will guide you through the design philosophies and guidelines for building software. The key focus of the course is to teach you how to handle and minimize complexity in your systems, ensuring they are robust and maintainable.

Throughout the course, you will delve into advanced Rust topics, such as Trait driven development, Rust generics, associated trait types, and procedural macros for code generation. It is important to have a good understanding of the basics of Rust before taking the course. If you're new to Rust, a great starting point is the [Rust book](https://doc.rust-lang.org/book/), which provides a solid foundation for learning the language.

By using this starter kit and completing the accompanying course, you'll be well-equipped to build scalable and maintainable services using Rust.

## Structure

- `app` contains crates producing binaries, such as the service and tooling.
- `business` contains business logic modules for runtimes.
- `foundation` contains foundational shared code.
- `runtime` contains the final runtime for the service
- `storage` contains crates for interaction with storage mechanisms.
- `transport` contains transport crates, for interacting with the outside world (JsonRPC, REST API, etc).

While the course and wiki are being developed, the documentation of each part should give a good overview of how parts are connected with each other.
