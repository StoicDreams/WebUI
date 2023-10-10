use crate::prelude::*;

#[function_component(MyFiStorageConcent)]
pub fn myfi_storage_concent() -> Html {
    let app_type = "website";
    let confirm_local_storage = {
        Callback::from(move |_| {
            user_accepts_local_storage();
        })
    };
    let confirm_session_storage = {
        Callback::from(move |_| {
            user_accepts_session_storage();
        })
    };
    let confirm_mem_storage = {
        Callback::from(move |_| {
            user_rejects_cached_storage();
        })
    };
    html! {
        <Paper>
            {title_primary!("Storage Concent")}
            <Paper>
                {"This website has multiple levels of data storage available for you to choose from, which will determine how data is stored on your device for this website."}
            </Paper>
            <Paper class="d-flex flex-column flex-wrap gap-1">
                <Card title="Memory-only Storage (default behavior - Maximum Security)" theme={Theme::Secondary}>
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
                        <Button onclick={confirm_mem_storage}>{"I confirm this is not my personal device or I simply want to assure a login is required everytime I access this website."}</Button>
                    </Paper>
                </Card>

                <Card title="Single Session/Tab Storage (Medium Security)" theme={Theme::Secondary}>
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
                        <Button onclick={confirm_session_storage}>{"I confirm that I am using my personal device and that I understand the above statements and I accept the use of storing my data in the browser so my login and other data will be remembered when the website is reloaded."}</Button>
                    </Paper>
                </Card>

                <Card title="Long-term Storage (Lowest Security)" theme={Theme::Secondary}>
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
                        <Button onclick={confirm_local_storage}>{"I confirm that I am using my personal device and that I understand the above statements and I accept the use of storing my data in the browser so my login and other data will be remembered any time I access this website."}</Button>
                    </Paper>
                </Card>
            </Paper>
        </Paper>
    }
}
