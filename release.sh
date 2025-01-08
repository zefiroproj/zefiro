#!/usr/bin/env bash

if ! command -v typos &>/dev/null; then
  echo "typos is not installed. Run 'cargo install typos-cli' to install it, otherwise the typos won't be fixed"
  exit 1
fi

if ! command -v git-cliff &>/dev/null; then
  echo "git-cliff is not installed. Run 'cargo install git-cliff' to install it, otherwise the CHANGELOG.md won't be updated"
  exit 1
fi

if [ -z "$1" ]; then
	echo "Please provide a tag."
	echo "Usage: ./release.sh v[X.Y.Z]"
	exit
fi

echo "Preparing $1..."

# update the version
msg="# managed by release.sh"
sed -E -i "s/^version = .* $msg$/version = \"${1#v}\" $msg/" zefiro*/Cargo.toml
sed -E -i "s/^version = .* $msg$/version = \"${1#v}\" $msg/" Cargo.toml

# update the changelog
git cliff --config cliff.toml --tag "$1" >CHANGELOG.md
git add -A && git commit -m "chore(release): prepare for $1"
git show

# generate a changelog for the tag message
export GIT_CLIFF_TEMPLATE="\
	{% for group, commits in commits | group_by(attribute=\"group\") %}
	{{ group | upper_first }}\
	{% for commit in commits %}
		- {% if commit.breaking %}(breaking) {% endif %}{{ commit.message | upper_first }} ({{ commit.id | truncate(length=7, end=\"\") }})\
	{% endfor %}
	{% endfor %}"
if [ ! -f "test_data/detailed.toml" ]; then
	echo "Error: test_data/detailed.toml not found"
	exit 1
fi
changelog=$(git cliff --config test_data/detailed.toml --unreleased --strip all)

# create a signed tag
git -c user.name="zefiro" \
	-c user.email="zefiroproj@protonmail.com" \
	-c user.signingkey="3ECB750FA11480F8DEF1E1B37614FD08EF9DB246" \
	tag -s -a "$1" -m "Release $1" -m "$changelog"
git tag -v "$1"
echo "Done!"
echo "Now push the commit (git push) and the tag (git push --tags)."