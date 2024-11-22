#!/bin/sh'
cd web
rm -r dist

cd src

echo "Copying production variables..."
rm env.ts
cp env.prod.ts env.ts

echo "Preparing to build..."
cd ..
bun run build

echo "Copying development variables"
cd src
rm env.ts
cp env.dev.ts env.ts

echo "Moving client files to core..."
cd ../..
rm -r core/web 
mv web/dist core/web

echo "Build complete!"
