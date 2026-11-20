// The `embassy` question is only asked without Wi-Fi.
if variable::get("wifi") {
    variable::set("embassy", true);
}
