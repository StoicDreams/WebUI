use crate::*;
use webui::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_terms() -> Html {
    set_title(format!("{} Terms & Conditions", COMPANY_SINGULAR));
    html! {
        <>
            <MarkdownContent href="/d/en-US/terms.md" />
            {title_secondary!(html!{format!("{} Terms & Conditions", COMPANY_SINGULAR)})}
            {title_tertiary!("1. Terms")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "By accessing this website, or any of our associated software, you are agreeing to be bound by these Terms and Conditions of Use, all applicable laws and regulations, and agree that you are responsible for compliance with any applicable local laws. If you do not agree with any of these terms, you are prohibited from using or accessing this site. The materials contained in this web site are protected by applicable copyright and trade mark law."
                )}
            </Paper>
            {title_tertiary!("2. Cookies & Browser Storage")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "Features of this website may requires the use of cookies and other browser storage technologies so that we may accurately identify unique visitors and provide a better experience on this website."
                )}
            </Paper>
            {title_tertiary!("3. Use License")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("Permission is granted to temporarily download one copy of the materials (information or software) on {} web sites and software for personal, non-commercial transitory viewing only. This is the grant of a license, not a transfer of title, and under this license you may not:", COMPANY_PLURAL)
                )}
                <List>
                    {list_items!(
                        "modify or copy the materials;",
                        "use the materials for any commercial purpose, or for any public display (commercial or non-commercial);",
                        format!("attempt to decompile or reverse engineer any software contained on {} web site;", COMPANY_PLURAL),
                        "remove any copyright or other proprietary notations from the materials;",
                        "or transfer the materials to another person or \"mirror\" the materials on any other server."
                    )}
                </List>
                {paragraphs!(
                    format!("This license shall automatically terminate if you violate any of these restrictions and may be terminated by {} at any time. Upon terminating your viewing of these materials or upon the termination of this license, you must destroy any downloaded materials in your possession whether in electronic or printed format.", COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("4. Disclaimer")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("The materials on {} web sites and software are provided \"as is\". {} makes no warranties, expressed or implied, and hereby disclaims and negates all other warranties, including without limitation, implied warranties or conditions of merchantability, fitness for a particular purpose, or non-infringement of intellectual property or other violation of rights. Further, {} does not warrant or make any representations concerning the accuracy, likely results, or reliability of the use of the materials on its Internet web site or otherwise relating to such materials or on any sites linked to this site.", COMPANY_PLURAL, COMPANY_SINGULAR, COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("5. Limitations")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("In no event shall {} or its suppliers be liable for any damages (including, without limitation, damages for loss of data or profit, or due to business interruption,) arising out of the use or inability to use the materials on {} Internet site, even if {} or a {} authorized representative has been notified orally or in writing of the possibility of such damage. Because some jurisdictions do not allow limitations on implied warranties, or limitations of liability for consequential or incidental damages, these limitations may not apply to you.", COMPANY_SINGULAR, COMPANY_PLURAL, COMPANY_SINGULAR, COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("6. Revisions and Errata")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("The materials appearing on {} web sites and software could include technical, typographical, or photographic errors. {} does not warrant that any of the materials on its web site are accurate, complete, or current. {} may make changes to the materials contained on its web site at any time without notice. {} does not, however, make any commitment to update the materials.", COMPANY_PLURAL, COMPANY_SINGULAR, COMPANY_SINGULAR, COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("7. Links")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("{} has not reviewed all of the sites linked to its Internet web site and is not responsible for the contents of any such linked site. The inclusion of any link does not imply endorsement by {} of the site. Use of any such linked web site is at the user's own risk.", COMPANY_SINGULAR, COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("8. Site Terms of Use Modifications<")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("{} may revise these terms of use for its web site at any time without notice. By using this web site you are agreeing to be bound by the then current version of these Terms and Conditions of Use.", COMPANY_SINGULAR)
                )}
            </Paper>
            {title_tertiary!("9. Governing Law")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("Any claim relating to {} web sites and software shall be governed by the laws of the United States without regard to its conflict of law provisions.", COMPANY_PLURAL),
                    "General Terms and Conditions applicable to Use of a Web Site."
                )}
            </Paper>
            {title_tertiary!("10. User Generated Content")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    format!("{} sites and software may contain user generated content. We may review content and periodically delete or alter content we believe to be inappropriate, offensive, or disrespectful. This is at our discretion, and we make no guarantees regarding the safety or accuracy of user generated content.", COMPANY_PLURAL)
                )}
            </Paper>
        </>
    }
}
