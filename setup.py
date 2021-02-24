from setuptools import find_packages, setup


setup(
    name="pokescript",
    version="0.1.0",
    author="Alex Wennerberg",
    author_email="alex@alexwennerberg.com",
    description="pokemon bot",
    packages=find_packages(),
    install_requires=[],
    include_package_data=True,
    entry_points={
        "console_scripts": [
            "pokescript=pokescript.cli:run",
        ] 
    },
    classifiers=[
        "Programming Language :: Python :: 3",
    ],
)

