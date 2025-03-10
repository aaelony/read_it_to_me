# read it to me

Use Amazon Polly to read text out loud and write to an MP3 file.
You may incur charges for usage of Amazon Polly.

e.g. The example text outputs [output_20250309_182255.mp3](output_20250309_182255.mp3)

## Setup

1. Clone the repository:
```bash
git clone https://github.com/USERNAME/REPO.git
cd REPO
```

2. Configure AWS credentials:
- Set up ~/.aws/credentials or
- Set environment variables:
  - AWS_ACCESS_KEY_ID
  - AWS_SECRET_ACCESS_KEY
  - AWS_REGION

3. Run the application:
```bash
cargo run
```

## Features

- Text-to-speech conversion using AWS Polly
- Neural voice support
- MP3 output with timestamp-based filenames
- Automatic handling of long text inputs

## Dependencies

- aws-sdk-polly
- tokio
- rodio
- chrono
