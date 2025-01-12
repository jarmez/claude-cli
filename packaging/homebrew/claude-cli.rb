class ClaudeCli < Formula
  desc "Command line interface for the Claude AI assistant"
  homepage "https://github.com/jarmez/claude-cli"
  url "https://github.com/jarmez/claude-cli/archive/v0.1.0.tar.gz"
  sha256 "UPDATE_WITH_ACTUAL_SHA256"
  license "MIT"

  depends_on "rust" => :build
  depends_on "openssl"

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/claude", "--version"
  end
end