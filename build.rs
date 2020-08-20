fn main() {
    #[cfg(windows)]
    {
        let target = std::env::var("TARGET").unwrap();
        if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
            for (key, value) in tool.env() {
                std::env::set_var(key, value);
            }
        }
        embed_resource::compile("resources/app.rc");
    }
}
