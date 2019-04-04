AC_INIT([hubauth], m4_esyscmd([grep ^version Cargo.toml | head -1 | awk '{print $3}' | tr -d '"' | tr -d "\n"]), [liam@ldaws.com])

PACKAGE_NAME=hubauth
VERSION=$(grep ^version Cargo.toml | head -1 | awk '{print $3}' | tr -d '"' | tr -d "\n")

AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
AS_IF(test x$CARGO = xno,
    AC_MSG_ERROR([cargo is required])
)
AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])
AS_IF(test x$RUSTC = xno,
    AC_MSG_ERROR([rustc is required])
)
CARGO_VENDOR=$( cargo vendor --version )
AS_IF([ test $? != 0 ], [AC_MSG_ERROR([cargo-vendor is required])])

dnl Release build we do.
CARGO_TARGET_DIR=release
AC_SUBST(CARGO_TARGET_DIR)
AC_SUBST(PACKAGE_NAME)
AC_SUBST(VERSION)

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
