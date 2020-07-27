#!/bin/bash
npm run-script build
zip -r9 function.zip node_modules
cd dist/
zip -g ../function.zip *.js
cd ..
echo "Deploying..."
aws lambda update-function-code --function-name up-webhook-ingester --zip-file fileb://function.zip --profile personalacc
rm function.zip
echo "Deployed to Lambda."