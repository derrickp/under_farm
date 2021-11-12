pub fn trim(value: String) -> String {
    value.replace("\"", "").replace("\\", "")
}
