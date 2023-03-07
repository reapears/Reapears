//! Confirmation code email sent to the user (existing email)
//! when they want to change their email.

use crate::{error::ServerResult, mail::EmailMessage};

/// Confirmation code email
///
/// # Errors
///
/// Return an error if failed to build an email
pub fn confirmation_code_email(
    first_name: &str,
    user_email: &str,
    subject: &str,
    code: &str,
) -> ServerResult<EmailMessage> {
    let text = confirmation_code_email_text(first_name, code);
    let html = confirmation_code_email_html(first_name, code);
    EmailMessage::from_outlook(user_email, subject, text, html)
}

/// Confirmation code email text
fn confirmation_code_email_text(first_name: &str, code: &str) -> String {
    format!(
        "
Hello, {first_name}!

Please use the following security code for the Reapears account email change

Security code: {code}

Thanks,
The Reapears team
    "
    )
}

/// Confirmation code email html
#[allow(clippy::too_many_lines)]
fn confirmation_code_email_html(first_name: &str, code: &str) -> String {
    format!(
        r##"

    <!doctype html>
    <html>
    
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
        <title></title>
        <style>
            img {{
                border: none;
                -ms-interpolation-mode: bicubic;
                max-width: 100%;
            }}
    
            body {{
                background-color: #f6f6f6;
                font-family: sans-serif;
                -webkit-font-smoothing: antialiased;
                font-size: 14px;
                line-height: 1.4;
                margin: 0;
                padding: 0;
                -ms-text-size-adjust: 100%;
                -webkit-text-size-adjust: 100%;
            }}
    
            table {{
                border-collapse: separate;
                mso-table-lspace: 0pt;
                mso-table-rspace: 0pt;
                width: 100%;
            }}
    
            table td {{
                font-family: sans-serif;
                font-size: 14px;
                vertical-align: top;
            }}
    
            .body {{
                background-color: #f6f6f6;
                width: 100%;
            }}
    
            .container {{
                display: block;
                margin: 0 auto !important;
                /* makes it centered */
                max-width: 580px;
                padding: 10px;
                width: 580px;
            }}
    
            .content {{
                box-sizing: border-box;
                display: block;
                margin: 0 auto;
                max-width: 580px;
                padding: 10px;
            }}
    
            .main {{
                background: #ffffff;
                border-radius: 3px;
                width: 100%;
            }}
    
            .wrapper {{
                box-sizing: border-box;
                padding: 20px;
            }}
    
            .content-block {{
                padding-bottom: 10px;
                padding-top: 10px;
            }}
    
            .footer {{
                clear: both;
                margin-top: 10px;
                text-align: center;
                width: 100%;
            }}
        
            p,
            ul,
            ol {{
                font-family: sans-serif;
                font-size: 14px;
                font-weight: normal;
                margin: 0;
                margin-bottom: 15px;
            }}
    
            p li,
            ul li,
            ol li {{
                list-style-position: inside;
                margin-left: 5px;
            }}
    
            .btn>tbody>tr>td {{
                padding-bottom: 15px;
            }}
    
            .btn table {{
                width: auto;
            }}
    
            .btn table td {{
                background-color: #ffffff;
                border-radius: 5px;
                text-align: center;
            }}
    
            .last {{
                margin-bottom: 0;
            }}
    
            .first {{
                margin-top: 0;
            }}
    
            .align-center {{
                text-align: center;
            }}
    
            .align-right {{
                text-align: right;
            }}
    
            .align-left {{
                text-align: left;
            }}
    
            .clear {{
                clear: both;
            }}
    
            .mt0 {{
                margin-top: 0;
            }}
    
            .mb0 {{
                margin-bottom: 0;
            }}
    
            .preheader {{
                color: transparent;
                display: none;
                height: 0;
                max-height: 0;
                max-width: 0;
                opacity: 0;
                overflow: hidden;
                mso-hide: all;
                visibility: hidden;
                width: 0;
            }}
    
            .powered-by a {{
                text-decoration: none;
            }}
    
            hr {{
                border: 0;
                border-bottom: 1px solid #f6f6f6;
                margin: 20px 0;
            }}
    
    
            @media only screen and (max-width: 620px) {{
                table.body h1 {{
                    font-size: 28px !important;
                    margin-bottom: 10px !important;
                }}
    
                table.body p,
                table.body ul,
                table.body ol,
                table.body td,
                table.body span,
                table.body a {{
                    font-size: 16px !important;
                }}
    
                table.body .wrapper,
                table.body .article {{
                    padding: 10px !important;
                }}
    
                table.body .content {{
                    padding: 0 !important;
                }}
    
                table.body .container {{
                    padding: 0 !important;
                    width: 100% !important;
                }}
    
                table.body .main{{
                    border-left-width: 0 !important;
                    border-radius: 0 !important;
                    border-right-width: 0 !important;
                }}
    
                table.body .btn a {{
                    width: 100% !important;
                }}
    
                table.body .img-responsive {{
                    height: auto !important;
                    max-width: 100% !important;
                    width: auto !important;
                }}
            }}
    
            @media all {{
                .ExternalClass {{
                    width: 100%;
                }}
    
                .ExternalClass,
                .ExternalClass p,
                .ExternalClass span,
                .ExternalClass font,
                .ExternalClass td,
                .ExternalClass div {{
                    line-height: 100%;
                }}
    
                .apple-link a {{
                    color: inherit !important;
                    font-family: inherit !important;
                    font-size: inherit !important;
                    font-weight: inherit !important;
                    line-height: inherit !important;
                    text-decoration: none !important;
                }}
    
                #MessageViewBody a {{
                    color: inherit;
                    text-decoration: none;
                    font-size: inherit;
                    font-family: inherit;
                    font-weight: inherit;
                    line-height: inherit;
                }}
            }}
        </style>
    </head>
    
    <body>
        <table role="presentation" border="0" cellpadding="0" cellspacing="0" class="body">
            <tr>
                <td>&nbsp;</td>
                <td class="container">
                    <div class="content">
    
                        <!-- START CENTERED WHITE CONTAINER -->
                        <table role="presentation" class="main">
    
                            <!-- START MAIN CONTENT AREA -->
                            <tr>
                                <td class="wrapper">
                                    <table role="presentation" border="0" cellpadding="0" cellspacing="0">
                                        <tr>
                                            <td>
    
                                                <p
                                                    style="font-family: -apple-system,BlinkMacSystemFont,&quot;Segoe UI&quot;,Helvetica,Arial,sans-serif,&quot;Apple Color Emoji&quot;,&quot;Segoe UI Emoji&quot; !important;">
                                                    Hello,
                                                    <strong
                                                        style="font-weight: 600; box-sizing: border-box;  ">{first_name}</strong>!
                                                </p>
                                                <p
                                                    style="font-family: -apple-system,BlinkMacSystemFont, &quot;Segoe UI&quot;,Helvetica,Arial,sans-serif,&quot;Apple Color Emoji&quot;,&quot;Segoe UI Emoji&quot; !important;">
                                                    Please use the following security code for the Reapears account email change</p>
                                                <p
                                                    style="font-family: -apple-system,BlinkMacSystemFont, &quot;Segoe UI&quot;,Helvetica,Arial,sans-serif,&quot;Apple Color Emoji&quot;,&quot;Segoe UI Emoji&quot; !important;">
                                                     Security codes: <strong
                                                    style="font-weight: 600; box-sizing: border-box;  ">{code}</strong></p>
                                                

                                                <p style="box-sizing: border-box; margin-top: 0; margin-bottom: 0px; font-family: -apple-system,BlinkMacSystemFont,&quot;Segoe UI&quot;,Helvetica,Arial,sans-serif,&quot;Apple Color Emoji&quot;,&quot;Segoe UI Emoji&quot; !important;">
                                                Thanks,
                                                </p>
                                                <p style="box-sizing: border-box; margin-top: 0; margin-bottom: 10px; font-family: -apple-system,BlinkMacSystemFont,&quot;Segoe UI&quot;,Helvetica,Arial,sans-serif,&quot;Apple Color Emoji&quot;,&quot;Segoe UI Emoji&quot; !important;">
                                                The Reapears team
                                                </p>
                                            </td>
                                        </tr>
                                    </table>
                                </td>
                            </tr>
                        </table>
    
                    </div>
                </td>
                <td>&nbsp;</td>
            </tr>
        </table>
    </body>
    
    </html>
    
    
    "##
    )
}

// 'Google email change verification' message

// If you've recently changed the email address on your Google Account, you should receive two emails with these subjects:

//     'Google Accounts: Email Change Verification': you get this one at your new email address. The email asks you to verify the change by clicking the provided link. If you didn't initiate the change, simply disregard the email.
//     'Google Accounts: Email Change Notification': you receive this one at your old email address. Please note that this email is a security measure to protect your account in case someone is trying to take it over.
