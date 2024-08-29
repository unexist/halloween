#-*- encoding: utf-8 -*-
#
# @package hw
#
# @file Sinatra handler
# @author Christoph Kappel <christoph@unexist.dev>
# @version $Id$
#
# This program can be distributed under the terms of the GNU GPLv3.
# See the file COPYING for details.
#

require "rubygems"
gem "sinatra", "1.4.8"
require "sinatra"
require "haml"
require "data_mapper"
require "json"

# DatamMpper
DataMapper.setup(:default, "sqlite3://#{Dir.pwd}/hw.db")

# Config
configure do
  set :port, 10000
  set :server, "webrick"
end

class Ghost
  include DataMapper::Resource

  property :id,         Serial
  property :name,       String
  property :tested,     Boolean
  property :vacced,     Boolean
  property :comment,    Text
  property :created_at, DateTime
end

DataMapper.finalize
Ghost.auto_upgrade!

# Routes
get "/" do
  @ghosts = Ghost.all(:order => [ :created_at.desc ])

  haml :index
end

post "/ghost" do
  Ghost.create(name: params["name"],
    tested: params["tested"],
    vacced: params["vacced"],
    comment: params["comment"],
    created_at: Time.now
  )

  200
end
