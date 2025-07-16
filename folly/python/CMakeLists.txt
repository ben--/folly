# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

include(FBPythonBinary)

set(
  FOLLY_PY_SRCS
#   build_mode.pyx
  iobuf.pyi
#   setup.py
#   build_mode.pyi
#   iobuf.pyx
#   windows/iocp.pyx
#   executor.pyx
#   fiber_manager.pyx 

  # test/teardown.py
  # test/simplebridge.pyx
  # test/coro.py
  # test/test_set_executor.py
  # test/iobuf_helper.pyx
  # test/simplebridgecoro.pyx
  # test/futures.py
  # test/generator.py
  # test/iobuf.py
  # test/iobuf_helper.pyi
  # test/simplegenerator.pyx
  # test/test_set_executor_cython.pyx
)

add_fb_python_library(
  folly_py
  SOURCES ${FOLLY_PY_SRCS}
  NAMESPACE "folly"
)
install_fb_python_library(
  folly_py
  EXPORT folly-exports
)

