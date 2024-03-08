FROM ghcr.io/bmrlab/muse-credits:v0.2 as github-credits


ENV task_params=" " start_params=" "
COPY --from=github-credits /usr/app/config /usr/app/config
COPY --from=github-credits /usr/app/credits-cli /usr/app/credits-cli
EXPOSE 8080
CMD ["sh", "-c", "cd /usr/app && ./credits-cli task ${task_params} && ${start_params} ./credits-cli start"]


