# kreuzcrawl-ffi CMake config-mode find module
#
# Defines the imported target:
#   kreuzcrawl-ffi::kreuzcrawl-ffi
#
# Usage:
#   find_package(kreuzcrawl-ffi REQUIRED)
#   target_link_libraries(myapp PRIVATE kreuzcrawl-ffi::kreuzcrawl-ffi)

if(TARGET kreuzcrawl-ffi::kreuzcrawl-ffi)
  return()
endif()

get_filename_component(_KREUZCRAWL_FFI_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_KREUZCRAWL_FFI_PREFIX "${_KREUZCRAWL_FFI_CMAKE_DIR}/.." ABSOLUTE)

# ── Step 1: Find the library and headers ──────────────────────────────

find_library(_KREUZCRAWL_FFI_LIBRARY
  NAMES kreuzcrawl_ffi libkreuzcrawl_ffi
  PATHS "${_KREUZCRAWL_FFI_PREFIX}/lib"
  NO_DEFAULT_PATH
)

if(NOT _KREUZCRAWL_FFI_LIBRARY)
  find_library(_KREUZCRAWL_FFI_LIBRARY
    NAMES kreuzcrawl_ffi libkreuzcrawl_ffi
  )
endif()

find_path(_KREUZCRAWL_FFI_INCLUDE_DIR
  NAMES kreuzcrawl.h
  PATHS "${_KREUZCRAWL_FFI_PREFIX}/include"
  NO_DEFAULT_PATH
)

if(NOT _KREUZCRAWL_FFI_INCLUDE_DIR)
  find_path(_KREUZCRAWL_FFI_INCLUDE_DIR NAMES kreuzcrawl.h)
endif()

# ── Step 2: Validate that required files were found ───────────────────

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(kreuzcrawl-ffi
  REQUIRED_VARS _KREUZCRAWL_FFI_LIBRARY _KREUZCRAWL_FFI_INCLUDE_DIR
)

# ── Step 3: Create the imported target with correct library type ──────

if(kreuzcrawl-ffi_FOUND)
  # Determine library type from the file extension
  set(_KREUZCRAWL_FFI_LIB_TYPE UNKNOWN)

  if(_KREUZCRAWL_FFI_LIBRARY MATCHES "\\.(dylib|so)$" OR _KREUZCRAWL_FFI_LIBRARY MATCHES "\\.so\\.")
    set(_KREUZCRAWL_FFI_LIB_TYPE SHARED)
  elseif(_KREUZCRAWL_FFI_LIBRARY MATCHES "\\.dll$")
    set(_KREUZCRAWL_FFI_LIB_TYPE SHARED)
  elseif(_KREUZCRAWL_FFI_LIBRARY MATCHES "\\.(a|lib)$")
    set(_KREUZCRAWL_FFI_LIB_TYPE STATIC)
  endif()

  add_library(kreuzcrawl-ffi::kreuzcrawl-ffi ${_KREUZCRAWL_FFI_LIB_TYPE} IMPORTED)

  # ── Step 4: Set target properties ─────────────────────────────────

  set_target_properties(kreuzcrawl-ffi::kreuzcrawl-ffi PROPERTIES
    IMPORTED_LOCATION "${_KREUZCRAWL_FFI_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${_KREUZCRAWL_FFI_INCLUDE_DIR}"
  )

  # On Windows with SHARED libraries, handle the DLL + import lib split
  if(WIN32 AND _KREUZCRAWL_FFI_LIB_TYPE STREQUAL "SHARED")
    # The found .dll.lib or .lib is the import library; find the actual DLL
    find_file(_KREUZCRAWL_FFI_DLL
      NAMES kreuzcrawl_ffi.dll libkreuzcrawl_ffi.dll
      PATHS "${_KREUZCRAWL_FFI_PREFIX}/bin" "${_KREUZCRAWL_FFI_PREFIX}/lib"
      NO_DEFAULT_PATH
    )
    if(_KREUZCRAWL_FFI_DLL)
      set_target_properties(kreuzcrawl-ffi::kreuzcrawl-ffi PROPERTIES
        IMPORTED_LOCATION "${_KREUZCRAWL_FFI_DLL}"
        IMPORTED_IMPLIB "${_KREUZCRAWL_FFI_LIBRARY}"
      )
    endif()
    unset(_KREUZCRAWL_FFI_DLL CACHE)
  endif()

  # Platform-specific link dependencies
  if(APPLE)
    set_property(TARGET kreuzcrawl-ffi::kreuzcrawl-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES "-framework CoreFoundation" "-framework Security" pthread)
  elseif(UNIX)
    set_property(TARGET kreuzcrawl-ffi::kreuzcrawl-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES pthread dl m)
  elseif(WIN32)
    set_property(TARGET kreuzcrawl-ffi::kreuzcrawl-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES ws2_32 userenv bcrypt)
  endif()

  unset(_KREUZCRAWL_FFI_LIB_TYPE)
endif()

mark_as_advanced(_KREUZCRAWL_FFI_LIBRARY _KREUZCRAWL_FFI_INCLUDE_DIR)
unset(_KREUZCRAWL_FFI_CMAKE_DIR)
unset(_KREUZCRAWL_FFI_PREFIX)
