if [ -z $1 ]; then
  echo "Missing parameter: service to be generated"
  exit 1
fi

function transform() {
  if [ ! -f ./services/$1/transform.js ]; then
    echo "Missing required transform.js file for service '$1'"
    exit 1
  fi

  node ./services/$1/transform.js
}

function postGen() {
  if [ -f ../../services/$1/post-gen.js ]; then
    node ../../services/$1/post-gen.js
  fi
}

# Run the transform scripts
transform $1

# Remove the previously generated typescript 
rm -rf gen/$1

# Generate the typescript bindings from the transformed open spec
openapi-generator-cli generate --input-spec ./services/$1/transformed_spec.yml --generator-name typescript-fetch --output gen/$1 --config ./services/$1/config.json && cd gen/$1 && postGen $1 && npm install && npm run build