use mlua::{Result, FromLuaMulti};

pub fn handle_result<'lua, T>(result: Result<T>)
    where
        T: FromLuaMulti<'lua>,
{
    if let Err(err) = result {
        error!("lua {err}");
    }
}
