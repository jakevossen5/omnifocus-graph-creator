from setuptools import setup

setup(
   name='omnifocus-graph-creator',
   version='1.0.6',
   description='Create graphs to analyze your OmniFocus data',
   author='Jake Vossen',
   author_email='jake@vossen.dev',
   packages=['omnifocus-graph-creator'],  #same as name
   install_requires=['matplotlib'], #external packages as dependencies
)
