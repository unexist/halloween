# -*- encoding: utf-8 -*-
#
# @package hw
#
# @file Sinatra handler
# @author Christoph Kappel <christoph@unexist.dev>
# @version $Id$
#
# This program can be distributed under the terms of the GNU GPLv2.
# See the file COPYING for details.
#

require "rubygems"
require "sinatra"
require "haml"
require "data_mapper"
require "json"

# DatamMpper
DataMapper.setup(:default, "sqlite3://#{Dir.pwd}/hw.db")

class Ghost
  include DataMapper::Resource

  property :id,         Serial
  property :name,       String
  property :brings,     Text
  property :created_at, DateTime
end

DataMapper.finalize
Ghost.auto_upgrade!

# Config
configure do
  set :port, 8000
end

# Routes
get "/" do
  @ghosts = Ghost.all(:order => [ :name.desc ])

  haml :index
end

post "/ghost" do
  Ghost.create(name: params["name"],
    brings: params["brings"], created_at: Time.now)

  200
end
