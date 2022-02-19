Lambda function with Tower Latency Injector
===========================================

This uses a [`tower::Layer`](https://docs.rs/tower/latest/tower/layer/trait.Layer.html) that has a 50% probability of adding 300-500ms latency to the Lambda function call.

## Usage

From the root of the project, run:

```bash
cargo build -p tower-lambda-injector --release
mkdir tower-lambda-injector/build
cp target/release/tower-lambda-injector tower-lambda-injector/build/bootstrap
```

From the `tower-lambda-injector` folder, deploy the Lambda function using [AWS SAM](https://github.com/aws/aws-sam-cli):

```bash
sam deploy -g
```

You can then invoke the Lambda function from the AWS Console or the AWS CLI, using this payload for the request:


```json
{
    "command": "test"
}
```