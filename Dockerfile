FROM ghcr.io/bmrlab/muse-credits:60d6f13 as github-credits

ENV task_params=" " start_params=" "

EXPOSE 8080
CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && ${start_params} /usr/app/credits-cli start"]


