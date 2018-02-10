# -*- coding: utf-8 -*-

import logging

logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)


def logging_flow():
    modules = ["init", "analytics"]
    logger.info("Starting Flow")
    if "init" in modules:
        logger.info("Loading init module..")
    else:
        logger.debug("Module init not found")
        raise Exception("Required module not found!")

    if "security" not in modules:
        logger.warning("Module security not found!")


if __name__ == "__main__":
    logging_flow()
    """ output:
    INFO:__main__:Starting Flow
    INFO:__main__:Loading init module..
    WARNING:__main__:Module security not found!
    """
