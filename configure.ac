AC_INIT(hubauth, m4_esyscmd([grep version Cargo.toml -m1 | awk '{print $3}' | tr -d '"' | tr -d "\n"]), liam@ldaws.com)

VERSION=$(grep version Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n")

# Default value for sbindir
prefix_temp=$prefix
exec_prefix_temp=$exec_prefix

test "${prefix}" = "NONE" && prefix="${ac_default_prefix}"
test "${exec_prefix}" = "NONE" && exec_prefix='${prefix}'

# Initial Value is $exec_prefix/sbin
sbintemp="${sbindir}"

# Expands to $prefix/sbin
eval sbintemp=\"${sbintemp}\"
# Expands to /usr/local/sbin or /usr/sbin if --prefix is passed
eval sbintemp=\"${sbintemp}\"
SBINDIR=${sbintemp}

AC_SUBST(SBINDIR)
AC_SUBST(VERSION)

AC_CONFIG_FILES([Makefile
                 hubauth.spec])

AC_OUTPUT
