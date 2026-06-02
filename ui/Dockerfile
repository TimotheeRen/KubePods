FROM node:24-alpine AS builder
WORKDIR /app
ENV CI=true
RUN npm install -g pnpm
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm config set minimum-release-age 0 \
  && pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build

FROM nginx:stable-alpine3.23 AS runner
WORKDIR /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
