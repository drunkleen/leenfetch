Name:           leenfetch
Version:        1.0.4
Release:        1%{?dist}
Summary:        a modern, minimal, and the fastest system info tool, written in Rust.

License:        MIT
URL:            https://github.com/drunkleen/leenfetch
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz
BuildRequires:  cargo
BuildRequires:  rust-packaging

%description
Leenfetch is a fast and modern Neofetch alternative written in Rust,
displaying minimal yet useful system information, fully customizable.

%prep
%autosetup -n leenfetch-%{version}

%build
# Release build
cargo build --release

%install
# Install binary
install -Dm0755 target/release/leenfetch %{buildroot}%{_bindir}/leenfetch

# Install man page
install -Dm0644 leenfetch.1 %{buildroot}%{_mandir}/man1/leenfetch.1


%files
%license LICENSE
%doc README.md
%{_bindir}/leenfetch
%{_mandir}/man1/leenfetch.1.gz

%changelog
* Sat Oct 25 2025 DrunkLeen <snape@drunkleen.com> - 1.0.4-1
- Initial COPR release
