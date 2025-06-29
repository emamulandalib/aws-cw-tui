# Security Policy

## Supported Versions

We take security seriously and provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We appreciate your efforts to responsibly disclose security vulnerabilities. If you discover a security vulnerability, please follow these steps:

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please send an email to: **security@aws-cw-tui.dev**

### What to Include

Please include the following information in your report:

- **Description**: A clear description of the vulnerability
- **Impact**: The potential impact of the vulnerability
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Proof of Concept**: If possible, include a proof of concept
- **Environment**: Operating system, version, and any other relevant details
- **Suggested Fix**: If you have suggestions for how to fix the issue

### Response Timeline

We are committed to addressing security vulnerabilities promptly:

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Status Updates**: We will provide regular updates on our progress
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days

### Security Process

1. **Triage**: We will assess the severity and impact of the reported vulnerability
2. **Investigation**: We will investigate the issue and develop a fix
3. **Testing**: We will thoroughly test the fix to ensure it doesn't introduce new issues
4. **Release**: We will release a security update and publish a security advisory
5. **Disclosure**: We will coordinate disclosure with the reporter

### Severity Guidelines

We use the following severity levels:

- **Critical**: Can be exploited remotely or leads to system compromise
- **High**: Significant impact on security or privacy
- **Medium**: Moderate impact with limited scope
- **Low**: Minor security issue with minimal impact

### Recognition

By default, we will acknowledge security researchers who responsibly report vulnerabilities to us. If you prefer to remain anonymous, please let us know in your report.

### Security Best Practices

When using AWS CloudWatch TUI:

#### AWS Credentials
- Use IAM roles when possible instead of access keys
- Follow the principle of least privilege for IAM permissions
- Rotate access keys regularly
- Never commit AWS credentials to version control
- Use AWS CLI profiles or environment variables for credential management

#### Network Security
- Use VPC endpoints for AWS service communication when available
- Ensure proper network ACLs and security groups are configured
- Monitor CloudTrail logs for unusual API activity

#### Application Security
- Keep the application updated to the latest version
- Review IAM permissions granted to the application
- Monitor for suspicious AWS API usage patterns

### Dependencies

We regularly monitor our dependencies for known vulnerabilities and update them as needed. You can view our current dependencies in the `Cargo.toml` file.

### Security Updates

Security updates will be released as patch versions and announced through:
- GitHub Security Advisories
- Release notes
- Email notification to security contact (if provided)

## Contact

For general security questions or concerns, please contact us at security@aws-cw-tui.dev.

---

Thank you for helping keep AWS CloudWatch TUI and its users secure!