use crate::prelude::*;

pub fn render_myfi_storage_consent(_contexts: Contexts) -> Html {
    html!(
        <MyFiStorageConsent />
    )
}

#[function_component(MyFiStorageConsent)]
pub fn myfi_storage_consent() -> Html {
    let app_type = "website";
    let current_setting = use_state(|| get_user_storage_data(String::from("storage_accepted")));
    let confirm_local_storage = {
        let current_setting = current_setting.clone();
        Callback::from(move |_| {
            user_accepts_local_storage();
            current_setting.set(String::from("2"));
        })
    };
    let confirm_session_storage = {
        let current_setting = current_setting.clone();
        Callback::from(move |_| {
            user_accepts_session_storage();
            current_setting.set(String::from("1"));
        })
    };
    let confirm_mem_storage = {
        let current_setting = current_setting.clone();
        Callback::from(move |_| {
            user_rejects_cached_storage();
            current_setting.set(String::from("0"));
        })
    };
    let card_width = 500;
    let mut btn_mem_theme = Theme::Warning;
    let mut btn_ses_theme = Theme::Warning;
    let mut btn_loc_theme = Theme::Warning;
    match current_setting.as_str() {
        "2" => {
            btn_loc_theme = Theme::Active;
        }
        "1" => {
            btn_ses_theme = Theme::Active;
        }
        _ => {
            btn_mem_theme = Theme::Active;
        }
    }
    html! {
        <>
            {title_primary!("Storage Concent")}
            <Paper elevation={ELEVATION_STANDARD} class={CLASSES_PAGE_SECTION}>
                <Paper>
                    {"This website has multiple levels of data storage available for you to choose from, which will determine how data is stored on your device for this website."}
                </Paper>
                <Paper class={CLASSES_CARD_CONTAINER}>
                    <Card width={card_width} title="Memory-only Storage (default behavior - Maximum Security)" theme={Theme::Secondary}>
                        <Paper>{"With this storage option, any login, settings, or other persistable information will only be retained in memory, and will be gone when the website is either closed or refreshed."}</Paper>
                        <Paper>{"Use this option when:"}</Paper>
                        <Paper>
                            <List>
                                <li>{"You are on a public computer, or some other device that is not yours."}</li>
                                <li>{"You do not want any personal data stored in any storage that will persist beyond a page refresh."}</li>
                                <li>{format!("You want to make sure you are required to login everytime you access this {}, even when you simply reload the page.", app_type)}</li>
                                <li>{"You have opted-in to storing your data on this computer. Selecting this option now will result in clearing any data saved from storage."}</li>
                            </List>
                        </Paper>
                        <Paper>
                            <Button color={btn_mem_theme} onclick={confirm_mem_storage}>{"I confirm this is not my personal device or I simply want to assure a login is required everytime I access this website."}</Button>
                        </Paper>
                    </Card>

                    <Card width={card_width} title="Single Session/Tab Storage (Medium Security)" theme={Theme::Secondary}>
                        <Paper>{"With this storage option, any time you open a new tab and visit this website, you will need to consent to storage and login in order to access your account features."}</Paper>
                        <Paper>{"Use this option when:"}</Paper>
                        <Paper>
                            <List>
                                <li>{"You are on your personal device."}</li>
                                <li>{"You will logout before leaving your device accessible to others."}</li>
                                <li>{format!("You want to stay logged in if you refresh the page, but prefer to login everytime you visit this {} in a new tab or window.", app_type)}</li>
                            </List>
                        </Paper>
                        <Paper>
                            <Button color={btn_ses_theme} onclick={confirm_session_storage}>{"I confirm that I am using my personal device and that I understand the above statements and I accept the use of storing my data in the browser so my login and other data will be remembered when the website is reloaded."}</Button>
                        </Paper>
                    </Card>

                    <Card width={card_width} title="Long-term Storage (Lowest Security)" theme={Theme::Secondary}>
                        <Paper>{"With this storage option, you only need to consent and login once. Your login will persist even after closing and reopening this website."}</Paper>
                        <Paper>{"Use this option when:"}</Paper>
                        <Paper>
                            <List>
                                <li>{"You are on your personal device."}</li>
                                <li>{"Your device is secure from other people accessing it."}</li>
                                <li>{"You want to remain logged in until you explicitely logout."}</li>
                            </List>
                        </Paper>
                        <Paper>
                            <Button color={btn_loc_theme} onclick={confirm_local_storage}>{"I confirm that I am using my personal device and that I understand the above statements and I accept the use of storing my data in the browser so my login and other data will be remembered any time I access this website."}</Button>
                        </Paper>
                    </Card>
                </Paper>
            </Paper>
        </>
    }
}
