FROM node:20-alpine

WORKDIR /usr/src/app

COPY package.json ./
COPY yarn.lock ./

COPY . .

RUN npm install

RUN yarn run build

EXPOSE 3000

CMD ["yarn", "preview", "--host", "0.0.0.0", "--port", "3000"]