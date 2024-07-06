file(GLOB_RECURSE CURRENT_HEADERS *.h *.hpp *.cuh)
file(GLOB_RECURSE CURRENT_SOURCES *.cpp *.cu *.c)

source_group("Include" FILES ${CURRENT_HEADERS})
source_group("Source" FILES ${CURRENT_SOURCES})

add_library(Foo STATIC ${CURRENT_HEADERS} ${CURRENT_SOURCES})

install(TARGETS Foo
  EXPORT FooTargets
  LIBRARY DESTINATION lib
  ARCHIVE DESTINATION lib
  RUNTIME DESTINATION bin
  INCLUDES DESTINATION include
)

file(GLOB MAIN_DIR_HEADERS *.h *.hpp *.cuh)
file(GLOB UTILS_HEADERS utils/*.h *.hpp *.cuh)

install(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/ DESTINATION include/foo
        FILES_MATCHING PATTERN "*.h" PATTERN "*.hpp" PATTERN "*.cuh")