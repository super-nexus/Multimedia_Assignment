# Use the official Node.js 14 image as a parent image
FROM node:16

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json (or yarn.lock) into the container
COPY package*.json ./

# Install dependencies in the container
RUN npm install

# Copy the rest of your app's source code from your host to your image filesystem.
COPY . .

# Make port available to the world outside this container
EXPOSE 3000

# Run the app when the container launches
CMD ["node", "index.js"]
