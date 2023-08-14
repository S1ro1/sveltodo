FROM node:20

# Set the working directory
WORKDIR /usr/src/app

# Copy the package.json and package-lock.json first, for better caching
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the project
COPY . .

# Expose the port the app runs on
EXPOSE 5173

# Command to run the application
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
