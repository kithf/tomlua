use mlua::{Lua, Result, Table, Value as LValue, Error, LuaSerdeExt, Nil, ExternalError};
use toml_edit::easy::Value as TValue;

fn decode<'a>(lua: &'a Lua, val: LValue<'a>) -> Result<LValue<'a>> {
  let val = match val {
    LValue::String(ref val) => Ok(val.to_string_lossy().to_string()),
    _ => Err(format!("Expected string, got {}", val.type_name()).to_lua_err()),
  }?;

  let doc = toml_edit::de::from_str::<TValue>(&val)
    .map_err(|e| Error::external(e.to_string()))?;
  lua.to_value(&doc)
}

fn encode<'a>(lua: &'a Lua, val: LValue<'a>) -> Result<LValue<'a>> {
  let toml = toml_edit::ser::to_string_pretty(&val)
      .map_err(|e| Error::external(e.to_string()))?;
  lua.create_string(&toml).map(LValue::String)
}

#[mlua::lua_module]
fn tomlua(lua: &Lua) -> Result<Table> {
  let decode = lua.create_function(|lua, s| match decode(lua, s) {
    Ok(val) => Ok((val, None)),
    Err(err) => Ok((Nil, Some(err.to_string())))
  })?;
  let encode = lua.create_function(|lua, s| match encode(lua, s) {
    Ok(val) => Ok((val, None)),
    Err(err) => Ok((Nil, Some(err.to_string())))
  })?;

  let exports = lua.create_table()?;
  exports.set("decode", decode)?;
  exports.set("encode", encode)?;
  Ok(exports)
}