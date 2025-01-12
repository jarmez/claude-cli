Name:           claude-cli
Version:        0.1.0
Release:        1%{?dist}
Summary:        Command line interface for the Claude AI assistant

License:        MIT
URL:            https://github.com/jarmez/claude-cli
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc
Requires:       openssl

%description
A command-line interface for the Claude AI assistant with Vim/Neovim-like interaction.

%prep
%autosetup

%build
cargo build --release

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p %{buildroot}%{_bindir}
install -m 755 target/release/claude %{buildroot}%{_bindir}/claude
install -m 755 target/release/claude-config %{buildroot}%{_bindir}/claude-config

mkdir -p %{buildroot}%{_sysconfdir}/claude-cli
mkdir -p %{buildroot}%{_datadir}/doc/claude-cli

%files
%license LICENSE
%doc README.md
%{_bindir}/claude
%{_bindir}/claude-config
%dir %{_sysconfdir}/claude-cli
%dir %{_datadir}/doc/claude-cli

%changelog
* Sun Jan 12 2025 James Day <james@day.net.nz> - 0.1.0-1
- Initial package release