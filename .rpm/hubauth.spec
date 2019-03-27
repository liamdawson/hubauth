%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: hubauth
Summary: Use remote sources for SSH authorized_keys, caching for network unavailability
Version: @@VERSION@@
Release: 1
License: MIT or ASL 2.0
Source0: %{name}-%{version}.tar.gz
Packager: Liam Dawson <liam@ldaws.com>

Requires(pre): shadow-utils

%description
%{summary}

%prep
%setup -q

%pre
getent group hubauth >/dev/null || groupadd -r hubauth
getent passwd hubauth >/dev/null || \
    useradd -r -g hubauth -d /var/cache/hubauth -s /sbin/nologin \
    -c "Runs hubauth and handles key caching" hubauth

%install
mkdir -p %{buildroot}
cp -a * %{buildroot}
mkdir -p %{buildroot}/var/cache/hubauth

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
%{_mandir}/man5/hubauth.yml.5*
%{_mandir}/man1/hubauth.1*
%{_docdir}/*
%config(noreplace) %attr(0600, hubauth, hubauth) /etc/hubauth.yml
%dir %attr(2700, hubauth, hubauth) /var/cache/hubauth
