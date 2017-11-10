import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def demo():
    logger.info("Agent sample -> started")
    logger.info("WARNING: agent sample not implemented")
    logger.info("Agent sample -> completed")
