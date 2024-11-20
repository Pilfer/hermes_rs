pub type JenkinsHash = u32;

pub const JENKINS_HASH_INIT: JenkinsHash = 0;

pub fn jenkins_add(hash: JenkinsHash, c: JenkinsHash) -> JenkinsHash {
    hash.wrapping_add(c)
}

pub fn jenkins_mix1(hash: JenkinsHash) -> JenkinsHash {
    hash.wrapping_add(hash << 10)
}

pub fn jenkins_mix2(hash: JenkinsHash) -> JenkinsHash {
    hash ^ (hash >> 6)
}

pub fn update_jenkins_hash<CharT>(hash: JenkinsHash, c: CharT) -> JenkinsHash
where
    CharT: Into<JenkinsHash>,
{
    jenkins_mix2(jenkins_mix1(jenkins_add(hash, c.into())))
}

pub fn hash_string(input: &str) -> JenkinsHash {
    let mut hash = JENKINS_HASH_INIT;
    for c in input.chars() {
        hash = update_jenkins_hash(hash, c as JenkinsHash);
    }
    hash
}
