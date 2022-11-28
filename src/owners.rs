
use users::{uid_t, gid_t, get_user_by_uid, get_group_by_gid};

pub fn get_user_name(uid: u32) -> Option<String>{
	let user = get_user_by_uid(uid as uid_t)?;

	Some(user.name().to_str()?.to_string())
}

pub fn get_group_name(gid: u32) -> Option<String>{
	let group = get_group_by_gid(gid as gid_t)?;

	Some(group.name().to_str()?.to_string())
}