FROM ghcr.io/bmrlab/muse-credits:v1.1 as github-credits

ENV start_params=" "

EXPOSE 8080
CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]

