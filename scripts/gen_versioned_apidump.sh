#!/bin/bash -e
# based on https://gist.github.com/magnetikonline/5faab765cf0775ea70cd2aa38bd70432
# heavily
function getRepoDumpSHA1List {
	pushd "$1" >/dev/null
	git rev-list --author="Roblox Client Tracker" --since="Tue Aug 8 23:44:11 2017 -0500" --until="Fri Feb 17 15:15:30 2023 -0600" --all --reverse
	popd >/dev/null
}
function getRepoFullDumpSHA1List {
	pushd "$1" >/dev/null
	git rev-list --author="Roblox Client Tracker" --since="Fri Feb 17 15:15:31 2023 -0600" --all --reverse
	popd >/dev/null
}

function exportDumpCommits {
	local commitSHA1

	local IFS=$'\n'
	local outputDir="${2%%/}"
	for commitSHA1 in $(getRepoDumpSHA1List "$1"); do
		# build export directory for commit and create
		local version=$(git --git-dir "$1/.git" show "$commitSHA1:version.txt")
		local exportFile="API-Dump-$version.json"
		echo "Export $commitSHA1 -> $outputDir/$exportFile"

		# create archive from commit then unpack to export directory
		git --git-dir "$1/.git" show "$commitSHA1:API-Dump.json" > $outputDir/$exportFile
	done
}
function exportFullDumpCommits {
	local commitSHA1

	local IFS=$'\n'
	local outputDir="${2%%/}"
	for commitSHA1 in $(getRepoFullDumpSHA1List "$1"); do
		# build export directory for commit and create
		local version=$(git --git-dir "$1/.git" show "$commitSHA1:version.txt")
		local exportFile="API-Dump-$version.json"
		echo "Export $commitSHA1 -> $outputDir/$exportFile"

		# create archive from commit then unpack to export directory
		git --git-dir "$1/.git" show "$commitSHA1:Full-API-Dump.json" > $outputDir/$exportFile
	done
}


# verify arguments
if [[ (! -d $1) || (! -d $2) ]]; then
	echo "Usage: $(basename "$0") GIT_DIR OUTPUT_DIR"
	exit 1
fi

# if [[ ! -d "$1/.git" ]]; then
# 	echo "Error: it seems [$1] is not a Git repository?" >&2
# 	exit 1
# fi

exportDumpCommits "$1" "$2"
exportFullDumpCommits "$1" "$2"
